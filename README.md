# gos

English| [简体中文](README.zh_cn.md)

[![crates.io](https://img.shields.io/crates/v/gos.svg)](https://crates.io/crates/gos)
[![Docs](https://docs.rs/gos/badge.svg)](https://docs.rs/gos)
[![MSRV](https://img.shields.io/badge/rustc-1.78.0+-ab6000.svg)]

gos is a tool written in Rust that helps you quickly create a Go project by specifying the Go modules you want to use in a config file. It automatically writes the import statements in the main.go file and runs go mod init and go mod tidy for you, initializing and updating your module dependencies.



## Installation

To install gos, you need to have Rust and Cargo installed on your system. You can follow the instructions here to install them.

Then, you can use Cargo to install gos from crates.io:

```bash
cargo install gos
```

Or, you can also clone this repository and build it from source:

```bash
git clone https://github.com/limitcool/gos.git
cd gos
cargo build --release
```

The executable file will be located in the `target/release` directory.

## Usage

The first time you run gos, it will create a config file named `config.yaml` in the current directory, where you need to fill in the Go modules you want to use. The config file has the following format:

```yaml
# The Go modules you want to use
Mods: ["github.com/charmbracelet/log"]
#  Whether to automatically create Vscode launch.json
CreateVscodeLaunch: true
```

You can specify any modules you want, as long as they are valid Go modules.

Then, you can run gos in the same directory to create a Go project:

```bash
gos new <project_name>
```

This will create a Go project with the name and modules you specified in the config file. It will also generate a `main.go` file with the import statements and a simple `main` function. For example, for the above config file, the `main.go` file might look like this:

```go
package main

import (
 "github.com/charmbracelet/log"
)

func main() {
 log.Info("gos")
}
```

## License

gos is licensed under the GPL license.
