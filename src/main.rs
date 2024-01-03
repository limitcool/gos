use rust_embed::RustEmbed;
use std::fs::{self, create_dir, create_dir_all, write, File};
use std::io::Read;
use std::io::Write;
use tracing::info;

pub const VSCODE_LAUNCH: &str = r#"{
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

pub const GO_TEMPATE: &str = r#"package main

    import (
    )

    func main() {
        log.Print("gos")
    }"#;

use crate::config::Config;
mod config;
fn main() {
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(tracing::Level::INFO)
        // build but do not install the subscriber.
        .init();

    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    // 检查参数个数是否正确
    if args.len() != 3 || &args[1] != "new" {
        println!("Usage: gos new project_name");
        return;
    }
    // 获取 project_name
    let project_name = &args[2];
    // if let Some(proj_dirs) = ProjectDirs::from("com", "initcool", "gos") {
    //     info!("{:?}", proj_dirs.config_dir());
    //     // fs::create_dir_all(proj_dirs.config_dir()).unwrap();
    //     Config::new(proj_dirs.config_dir().join("/config").to_str().unwrap()).unwrap();
    //     // Lin: /home/alice/.config/barapp
    //     // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
    //     // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
    // }
    // 调用 create_project 函数
    match create_project(project_name, Config::new("config.yaml").unwrap()) {
        Ok(_) => {
            info!("All done")
        }
        Err(e) => tracing::error!("{}", e),
    }
}

#[derive(RustEmbed)]
#[folder = "template/"]
struct Asset;

fn create_project(project_name: &str, config: Config) -> Result<(), std::io::Error> {
    create_dir(project_name)?;
    // let go_temp = Asset::get("main.tpl").unwrap();
    let mut f = fs::File::create(&format!("{}/{}", project_name, "main.go"))?;
    f.write(&GO_TEMPATE.as_bytes())?;
    if config.create_vscode_launch {
        info!("Create Vscode launch.json");
        create_dir_all(format!("{}/{}", project_name, ".vscode"))?;
        let mut f = fs::File::create(&format!("{}/{}/{}", project_name, ".vscode", "launch.json"))?;
        f.write(&VSCODE_LAUNCH.as_bytes())?;
    }
    add_mod(config.mods, project_name);
    info!("Current directory is: {:?}", std::env::current_dir()?);
    std::env::set_current_dir(project_name)?;
    info!("Switched directory to: {:?}", std::env::current_dir()?);
    info!("Running \"go mod init {}\"", project_name);
    std::process::Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(project_name)
        .output()?;
    info!("Running \"go mod tidy\"");
    std::process::Command::new("go")
        .arg("mod")
        .arg("tidy")
        .output()?;
    Ok(())
}

fn add_mod(mods: Vec<String>, project_name: &str) -> () {
    let project_file_path = format!("{}/{}", project_name, "main.go");
    // 打开 main.tpl 文件
    let mut file = File::open(&project_file_path).expect("Failed to open file");
    // 创建一个空的字符串变量
    let mut content = String::new();
    // 读取文件内容到字符串变量中
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    mods.iter().for_each(|mod_name|
    // 查找  import ( 的位置
        if let Some(pos) = content.find("import (") {
            // 在 import ( 的末尾插入换行符和mod
            info!("Add mod \"{}\"",mod_name);
            content.insert_str(
                pos + "import (".len(),
                format!("\n\t\"{}\"", mod_name).as_str(),
            );
        }
    );

    // 将修改后的字符串变量写回到 main.tpl 文件中
    write(project_file_path, &content).expect("Failed to write file");
}
