# gos

[![crates.io](https://img.shields.io/crates/v/gos.svg)](https://crates.io/crates/gos)
[![Docs](https://docs.rs/gos/badge.svg)](https://docs.rs/gos)
[![MSRV](https://img.shields.io/badge/rustc-1.78.0+-ab6000.svg)]

gos是一个用Rust编写的工具，它可以帮助你快速创建一个Go项目，只需在一个配置文件中指定你想要使用的Go模块。它会自动在main.go文件中写入导入语句，并为你运行go mod init和go mod tidy，初始化和更新你的模块依赖。

## 安装

要安装gos，你需要在你的系统上安装Rust和Cargo。你可以按照[这里](https://www.rust-lang.org/tools/install)的指示来安装它们。

然后，你可以使用Cargo从crates.io安装gos：

```bash
cargo install gos
```

或者，你也可以克隆这个仓库并从源代码构建它：

```bash
git clone https://github.com/limitcool/gos.git
cd gos
cargo build --release
```

可执行文件将位于`target/release`目录中。

## 使用

首次运行gos，它会在当前目录下新建一个名为`config.yaml`的配置文件，你需要在这个文件中填写你想要使用的Go模块。配置文件的格式如下：

```yaml
# 你想要使用的Go模块
Mods: ["github.com/charmbracelet/log"]
# 是否自动创建 Vscode launch.json
CreateVscodeLaunch: true
```

你可以指定任何你想要的模块，只要它们是有效的Go模块。

然后，你可以在同一个目录中运行gos来新建Go项目：

```bash
gos new <project_name>
```

这将根据你在配置文件中指定的名称和模块创建一个Go项目。它还会生成一个带有导入语句和一个简单的`main`函数的`main.go`文件。例如，对于上面的配置文件，`main.go`文件可能是这样的：

```go
package main

import (
 "github.com/charmbracelet/log"
)

func main() {
 log.Info("gos")
}
```

## 许可

gos使用GPL许可证授权。
