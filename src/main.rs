use std::fs::{self, create_dir, create_dir_all, write, File};
use std::io::Read;
use std::io::Write;
use tracing::info;

mod constants;
mod license;

use crate::config::Config;
mod config;

const MAIN_GO_FILE: &str = "main.go";
const LICENSE_FILE: &str = "LICENSE";
const README_FILE: &str = "README.md";
const GITIGNORE_FILE: &str = ".gitignore";
const VS_CODE_DIR: &str = ".vscode";
const LAUNCH_JSON_FILE: &str = "launch.json";
const MOD_IMPORT_MARKER: &str = "import (";
fn main() {
    let gos_version: &str = env!("CARGO_PKG_VERSION");
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(tracing::Level::INFO)
        // build but do not install the subscriber.
        .init();

    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    // 检查参数个数是否正确
    if args.len() != 3 || &args[1] != "new" {
        if args.len() == 2 {
            if &args[1] == "version" || &args[1] == "--version" {
                print!("gos version: {}", gos_version)
            } else {
                println!("Usage: gos new project_name");
            }
        } else {
            println!("Usage: gos new project_name");
        }

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
    match create_project(project_name, Config::new().unwrap()) {
        Ok(_) => {
            info!("All done")
        }
        Err(e) => tracing::error!("{}", e),
    }
}

fn create_project(project_name: &str, config: Config) -> Result<(), std::io::Error> {
    create_dir(project_name)?;
    // let go_temp = Asset::get("main.tpl").unwrap();
    let mut f = fs::File::create(&format!("{}/{}", project_name, "main.go"))?;
    f.write(constants::go_template::CODE.as_bytes())?;
    if config.create_vscode_launch {
        info!("Create Vscode launch.json");
        create_dir_all(format!("{}/{}", project_name, VS_CODE_DIR))?;
        let mut f = fs::File::create(&format!(
            "{}/{}/{}",
            project_name, VS_CODE_DIR, LAUNCH_JSON_FILE
        ))?;
        f.write(constants::vscode_launch::CONFIG.as_bytes())?;
    }
    if config.create_license {
        create_license(project_name).unwrap();
    };
    if config.create_gitignore {
        create_gitignore(project_name).unwrap();
    };
    if config.create_readme {
        create_readme(project_name).unwrap();
    };
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

fn add_mod(mods: Vec<String>, project_name: &str) {
    let project_file_path = format!("{}/{}", project_name, MAIN_GO_FILE);
    // 打开 main.tpl 文件
    let mut file = File::open(&project_file_path).expect("Failed to open file");
    // 创建一个空的字符串变量
    let mut content = String::new();
    // 读取文件内容到字符串变量中
    file.read_to_string(&mut content)
        .expect("Failed to read file");
    mods.iter().for_each(|mod_name|
    // 查找  import ( 的位置
        if let Some(pos) = content.find(MOD_IMPORT_MARKER) {
            // 在 import ( 的末尾插入换行符和mod
            info!("Add mod \"{}\"",mod_name);
            content.insert_str(
                pos + MOD_IMPORT_MARKER.len(),
                format!("\n\t\"{}\"", mod_name).as_str(),
            );
        }
    );

    // 将修改后的字符串变量写回到 main.tpl 文件中
    write(project_file_path, &content).expect("Failed to write file");
}

fn create_license(project_name: &str) -> Result<(), std::io::Error> {
    info!("Create LICENSE");
    let mut f = fs::File::create(&format!("{}/{}", project_name, LICENSE_FILE))?;
    f.write(license::LICENSE_GPL.as_bytes())?;
    Ok(())
}

fn create_gitignore(project_name: &str) -> Result<(), std::io::Error> {
    info!("Create .gitignore");
    let mut f = fs::File::create(&format!("{}/{}", project_name, GITIGNORE_FILE))?;
    f.write(b"")?;
    Ok(())
}

fn create_readme(project_name: &str) -> Result<(), std::io::Error> {
    info!("Create README.md");
    let mut f = fs::File::create(&format!("{}/{}", project_name, README_FILE))?;
    f.write(format!("## {}\n\n", project_name).as_bytes())?;
    Ok(())
}
