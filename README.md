# DevUtils

<div align="center">

![DevUtils Logo](assets/icon.svg)

[![Release](https://github.com/lichthis/dev-utils/actions/workflows/release.yml/badge.svg)](https://github.com/lichthis/dev-utils/actions/workflows/release.yml)
[![License](https://img.shields.io/github/license/lichthis/dev-utils)](LICENSE)

A collection of essential development utilities built with Rust and Dioxus.

[Features](#features) • [Installation](#installation) • [Usage](#usage) • [Contributing](#contributing)

</div>


## Installation

### From Releases

Download the latest version for your platform from the [Releases](https://github.com/lichthis/dev-utils/releases) page:

- Windows: `devutils-windows.zip`
- macOS: `devutils-macos.tar.gz`
- Linux: `devutils-linux.tar.gz`

### From Source

```bash
# Clone the repository
git clone https://github.com/lichthis/dev-utils.git
cd dev-utils

# Build and run
cargo build --release
cargo run --release
```

## Usage

1. Launch DevUtils
2. Select the desired tool from the toolbar
3. Input your text in the left panel
4. See the results in real-time in the right panel

## Development

### Prerequisites

- Rust (latest stable version)
- Cargo

### Setup Development Environment

```bash
# Install dependencies
cargo install --path .

# Run in development mode
cargo run
```

### Building

```bash
# Build for release
cargo build --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) - UI Framework
- [Rust](https://www.rust-lang.org/) - Programming Language