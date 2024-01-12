pub mod vscode_launch {
    pub const CONFIG: &str = r#"{
        // Use IntelliSense to learn about possible attributes.
        // Hover to view descriptions of existing attributes.
        // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
        "version": "0.2.0",
        "configurations": [
            {
                "name": "Launch Package",
                "type": "go",
                "request": "launch",
                "mode": "auto",
                "program": "${workspaceFolder}/main.go"
            }
        ]
    }"#;
}

pub mod go_template {
    pub const CODE: &str = r#"package main

import (
)

func main() {
    log.Print("gos")
}"#;
}
