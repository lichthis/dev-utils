use dioxus::prelude::*;
use serde_json::{Value, to_string_pretty};
use json5;
use serde_yaml;
use urlencoding::{encode, decode};
use arboard::Clipboard;
use tokio::time::{sleep, Duration};

#[derive(PartialEq, Clone)]
enum OutputFormat {
    DoubleQuotes,
    SingleQuotes,
    NoQuotes,
    Yaml,
}

#[derive(PartialEq)]
enum InputFormat {
    JSON,
    YAML,
}

#[derive(Props, Clone, PartialEq)]
struct CopyButtonProps {
    text: String,
}

#[component]
fn CopyButton(props: CopyButtonProps) -> Element {
    let mut copy_status = use_signal(|| false);

    let handle_copy = move |_| {
        if let Ok(mut clipboard) = Clipboard::new() {
            let _ = clipboard.set_text(props.text.clone());
            copy_status.set(true);
            let mut status = copy_status.clone();
            let _ = use_future(move || async move {
                sleep(Duration::from_millis(2000)).await;
                status.set(false);
            });
        }
    };

    rsx! {
        button {
            class: "copy-button",
            onclick: handle_copy,
            disabled: *copy_status.read(),
            if *copy_status.read() {
                "Copied!"
            } else {
                "Copy"
            }
        }
    }
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut selected_tool = use_signal(|| "json");

    rsx! {
        style { "{include_str!(\"style.css\")}" }
        title { "DevUtils - Development Utilities" }
        div { 
            class: "container",
            div { 
                class: "tool-selector",
                button {
                    class: if *selected_tool.read() == "json" { "selected" } else { "" },
                    onclick: move |_| selected_tool.set("json"),
                    "JSON Formatter"
                }
                button {
                    class: if *selected_tool.read() == "url" { "selected" } else { "" },
                    onclick: move |_| selected_tool.set("url"),
                    "URL Encoder"
                }
            }
            div { 
                class: "tool-container",
                match *selected_tool.read() {
                    "json" => rsx! { JsonFormatter {} },
                    "url" => rsx! { UrlEncoder {} },
                    _ => rsx! { "Invalid tool selected" }
                }
            }
        }
    }
}

#[component]
fn JsonFormatter() -> Element {
    let mut input = use_signal(String::new);
    let mut output = use_signal(String::new);
    let mut input_format = use_signal(|| InputFormat::JSON);
    let mut output_format = use_signal(|| OutputFormat::DoubleQuotes);

    let mut handle_format = move |new_input: String| {
        if new_input.is_empty() {
            output.set(String::new());
            return;
        }

        let parsed: Value = match *input_format.read() {
            InputFormat::JSON => match json5::from_str(&new_input) {
                Ok(v) => v,
                Err(e) => {
                    output.set(format!("Error: {}", e));
                    return;
                }
            },
            InputFormat::YAML => match serde_yaml::from_str(&new_input) {
                Ok(v) => v,
                Err(e) => {
                    output.set(format!("Error: {}", e));
                    return;
                }
            }
        };

        let formatted = match *output_format.read() {
            OutputFormat::DoubleQuotes => to_string_pretty(&parsed).unwrap_or_default(),
            OutputFormat::SingleQuotes => to_string_pretty(&parsed)
                .unwrap_or_default()
                .replace('\"', "'"),
            OutputFormat::NoQuotes => to_string_pretty(&parsed)
                .unwrap_or_default()
                .replace('\"', ""),
            OutputFormat::Yaml => serde_yaml::to_string(&parsed).unwrap_or_default(),
        };
        output.set(formatted);
    };

    rsx! {
        div { 
            class: "format-container json-formatter",
            // 输入区域
            div { 
                class: "input-section",
                div {
                    class: "section-header",
                    label { "Input" }
                    select {
                        onchange: move |evt| {
                            input_format.set(match evt.value().as_str() {
                                "yaml" => InputFormat::YAML,
                                _ => InputFormat::JSON
                            });
                            handle_format(input.read().to_string());
                        },
                        option { value: "json", "JSON" }
                        option { value: "yaml", "YAML" }
                    }
                }
                div {
                    class: "content-wrapper",
                    textarea {
                        class: "content-area",
                        placeholder: "Enter text to format",
                        oninput: move |evt| {
                            let value = evt.value().to_string();
                            input.set(value.clone());
                            handle_format(value);
                        },
                        value: "{input}"
                    }
                }
            }
            // 输出区域
            div { 
                class: "output-section",
                div {
                    class: "section-header",
                    label { "Formatted Output" }
                    select {
                        onchange: move |evt| {
                            output_format.set(match evt.value().as_str() {
                                "single" => OutputFormat::SingleQuotes,
                                "none" => OutputFormat::NoQuotes,
                                "yaml" => OutputFormat::Yaml,
                                _ => OutputFormat::DoubleQuotes
                            });
                            handle_format(input.read().to_string());
                        },
                        option { value: "double", "Double Quotes" }
                        option { value: "single", "Single Quotes" }
                        option { value: "none", "No Quotes" }
                        option { value: "yaml", "YAML" }
                    }
                }
                div {
                    class: "content-wrapper",
                    textarea {
                        class: "content-area",
                        readonly: true,
                        value: "{output}"
                    }
                    CopyButton { text: output.read().to_string() }
                }
            }
        }
    }
}

#[component]
fn UrlEncoder() -> Element {
    let mut input = use_signal(String::new);
    let mut encoded = use_signal(String::new);
    let mut decoded = use_signal(String::new);

    let mut handle_input = move |new_input: String| {
        input.set(new_input.clone());
        encoded.set(encode(&new_input).into_owned());
        match decode(&new_input) {
            Ok(text) => decoded.set(text.into_owned()),
            Err(e) => decoded.set(format!("Error: {}", e)),
        }
    };

    rsx! {
        div { 
            class: "format-container url-encoder",
            // 输入区域
            div { 
                class: "input-section",
                div {
                    class: "section-header",
                    label { "Input" }
                }
                div {
                    class: "content-wrapper",
                    textarea {
                        class: "content-area",
                        placeholder: "Enter text to encode/decode",
                        oninput: move |evt| handle_input(evt.value().to_string()),
                        value: "{input}"
                    }
                }
            }
            // 输出区域
            div { 
                class: "output-section",
                // URL Encoded 输出
                div {
                    class: "section-header",
                    label { "URL Encoded" }
                }
                div {
                    class: "content-wrapper",
                    textarea {
                        class: "content-area",
                        readonly: true,
                        value: "{encoded}"
                    }
                    CopyButton { text: encoded.read().to_string() }
                }
                // URL Decoded 输出
                div {
                    class: "section-header",
                    label { "URL Decoded" }
                }
                div {
                    class: "content-wrapper",
                    textarea {
                        class: "content-area",
                        readonly: true,
                        value: "{decoded}"
                    }
                    CopyButton { text: decoded.read().to_string() }
                }
            }
        }
    }
}