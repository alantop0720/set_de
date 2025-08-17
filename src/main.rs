use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const DEFAULT_MIRROR: &str =
    "https://ghfast.top/https://github.com/astral-sh/python-build-standalone/releases/download";
const ENV_VAR_NAME: &str = "UV_PYTHON_INSTALL_MIRROR";

const DEFAULT_INDEX: &str = "https://pypi.tuna.tsinghua.edu.cn/simple";
const INDEX_VAR_NAME: &str = "UV_DEFAULT_INDEX";

const PIP_CONFIG_CONTENT: &str = "[global]\nindex-url=https://pypi.tuna.tsinghua.edu.cn/simple\n\n[install]\ntrusted-host=pypi.tuna.tsinghua.edu.cn\n";

const UV_TOML_INDEX_CONFIG: &str = "[[index]]\nurl = \"https://mirrors.aliyun.com/pypi/simple/\"\ndefault = true\n# 或使用清华源\n# url = \"https://pypi.tuna.tsinghua.edu.cn/simple/\"";

fn main() {
    loop {
        show_menu();
        let choice = get_user_choice();

        match choice {
            1 => read_uv_python_install_mirror(),
            2 => set_uv_python_install_mirror_permanently(),
            3 => read_uv_default_index(),
            4 => set_uv_default_index_permanently(),
            5 => read_pip_ini(),
            6 => write_pip_ini(),
            7 => read_uv_toml(),
            8 => write_uv_toml(),
            9 => {
                println!("退出程序");
                break;
            }
            _ => println!("无效选择，请重新输入"),
        }

        println!("\n按回车键继续...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

fn show_menu() {
    println!("==============================");
    println!("     环境变量设置程序");
    println!("==============================");
    println!("1. 读取 {} 环境变量", ENV_VAR_NAME);
    println!("2. 永久设置 {} 环境变量", ENV_VAR_NAME);
    println!("3. 读取 {} 环境变量", INDEX_VAR_NAME);
    println!("4. 永久设置 {} 环境变量", INDEX_VAR_NAME);
    println!("5. 读取 pip.ini 文件内容");
    println!("6. 写入 pip.ini 文件内容");
    println!("7. 读取 uv.toml 文件内容");
    println!("8. 写入 uv 索引配置到 uv.toml 文件");
    println!("9. 退出");
    println!("==============================");
    println!("请选择操作:");
}

fn get_user_choice() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    let choice: u32 = input.trim().parse().unwrap_or(0);
    choice
}

fn read_uv_python_install_mirror() {
    match env::var(ENV_VAR_NAME) {
        Ok(val) => {
            println!("当前 {} 环境变量值为:", ENV_VAR_NAME);
            println!("{}", val);
        }
        Err(_) => {
            println!("未设置 {} 环境变量", ENV_VAR_NAME);
        }
    }
}

fn set_uv_python_install_mirror_permanently() {
    println!("正在永久设置 {} 环境变量...", ENV_VAR_NAME);
    let os = env::consts::OS;

    let result = match os {
        "windows" => set_env_var_windows(ENV_VAR_NAME, DEFAULT_MIRROR),
        "linux" | "macos" => set_env_var_unix(ENV_VAR_NAME, DEFAULT_MIRROR),
        _ => {
            println!("不支持的操作系统: {}", os);
            println!("仅支持 Windows、Linux 和 macOS");
            return;
        }
    };

    if result {
        println!("✓ 环境变量已成功永久设置");
        println!("新设置将在新打开的终端/命令行窗口中生效");
    } else {
        println!("✗ 设置环境变量失败");
        println!("请尝试手动设置或以管理员权限运行程序");
    }
}

fn read_uv_default_index() {
    match env::var(INDEX_VAR_NAME) {
        Ok(val) => {
            println!("当前 {} 环境变量值为:", INDEX_VAR_NAME);
            println!("{}", val);
        }
        Err(_) => {
            println!("未设置 {} 环境变量", INDEX_VAR_NAME);
        }
    }
}

fn set_uv_default_index_permanently() {
    println!("正在永久设置 {} 环境变量...", INDEX_VAR_NAME);
    let os = env::consts::OS;

    let result = match os {
        "windows" => set_env_var_windows(INDEX_VAR_NAME, DEFAULT_INDEX),
        "linux" | "macos" => set_env_var_unix(INDEX_VAR_NAME, DEFAULT_INDEX),
        _ => {
            println!("不支持的操作系统: {}", os);
            println!("仅支持 Windows、Linux 和 macOS");
            return;
        }
    };

    if result {
        println!("✓ 环境变量已成功永久设置");
        println!("新设置将在新打开的终端/命令行窗口中生效");
    } else {
        println!("✗ 设置环境变量失败");
        println!("请尝试手动设置或以管理员权限运行程序");
    }
}

fn read_pip_ini() {
    // 只在 Windows 系统上查找 pip.ini 文件
    if env::consts::OS != "windows" {
        println!("此功能仅支持 Windows 系统");
        return;
    }

    // 获取当前用户的用户名
    let username = env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());

    // 构建 pip.ini 文件路径
    let pip_ini_path = format!("C:\\Users\\{}\\pip\\pip.ini", username);

    println!("正在读取文件: {}", pip_ini_path);

    match fs::read_to_string(&pip_ini_path) {
        Ok(content) => {
            println!("\n文件内容:");
            println!("{}", content);
        }
        Err(e) => {
            println!("读取文件失败: {}", e);
            println!("文件可能不存在或无权限访问");
        }
    }
}

fn write_pip_ini() {
    // 只在 Windows 系统上操作 pip.ini 文件
    if env::consts::OS != "windows" {
        println!("此功能仅支持 Windows 系统");
        return;
    }

    // 获取当前用户的用户名
    let username = env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());

    // 构建 pip.ini 文件路径
    let pip_ini_path = format!("C:\\Users\\{}\\pip\\pip.ini", username);

    println!("正在写入文件: {}", pip_ini_path);

    // 确保目录存在
    let pip_dir = format!("C:\\Users\\{}\\pip", username);
    let path = Path::new(&pip_dir);

    if !path.exists() {
        match fs::create_dir_all(&pip_dir) {
            Ok(_) => println!("已创建目录: {}", pip_dir),
            Err(e) => {
                println!("创建目录失败: {}", e);
                return;
            }
        }
    }

    // 写入文件内容
    match fs::File::create(&pip_ini_path) {
        Ok(mut file) => match file.write_all(PIP_CONFIG_CONTENT.as_bytes()) {
            Ok(_) => {
                println!("✓ 成功写入 pip.ini 文件");
                println!("\n写入的内容:");
                println!("{}", PIP_CONFIG_CONTENT);
            }
            Err(e) => {
                println!("写入文件失败: {}", e);
            }
        },
        Err(e) => {
            println!("创建文件失败: {}", e);
        }
    }
}

fn read_uv_toml() {
    // 只在 Windows 系统上查找 uv.toml 文件
    if env::consts::OS != "windows" {
        println!("此功能仅支持 Windows 系统");
        return;
    }

    // 获取当前用户的用户名
    let username = env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());

    // 构建 uv.toml 文件路径
    let uv_toml_path = format!("C:\\Users\\{}\\AppData\\Roaming\\uv\\uv.toml", username);

    println!("正在读取文件: {}", uv_toml_path);

    match fs::read_to_string(&uv_toml_path) {
        Ok(content) => {
            println!("\n文件内容:");
            println!("{}", content);
        }
        Err(e) => {
            println!("读取文件失败: {}", e);
            println!("文件可能不存在或无权限访问");
        }
    }
}

fn write_uv_toml() {
    // 只在 Windows 系统上操作 uv.toml 文件
    if env::consts::OS != "windows" {
        println!("此功能仅支持 Windows 系统");
        return;
    }

    // 获取当前用户的用户名
    let username = env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());

    // 构建 uv.toml 文件路径
    let uv_toml_path = format!("C:\\Users\\{}\\AppData\\Roaming\\uv\\uv.toml", username);

    println!("正在写入文件: {}", uv_toml_path);

    // 确保目录存在
    let uv_dir = format!("C:\\Users\\{}\\AppData\\Roaming\\uv", username);
    let path = Path::new(&uv_dir);

    if !path.exists() {
        match fs::create_dir_all(&uv_dir) {
            Ok(_) => println!("已创建目录: {}", uv_dir),
            Err(e) => {
                println!("创建目录失败: {}", e);
                return;
            }
        }
    }

    // 写入文件内容
    match fs::File::create(&uv_toml_path) {
        Ok(mut file) => match file.write_all(UV_TOML_INDEX_CONFIG.as_bytes()) {
            Ok(_) => {
                println!("✓ 成功写入 uv.toml 文件");
                println!("\n写入的内容:");
                println!("{}", UV_TOML_INDEX_CONFIG);
            }
            Err(e) => {
                println!("写入文件失败: {}", e);
            }
        },
        Err(e) => {
            println!("创建文件失败: {}", e);
        }
    }
}

fn set_env_var_windows(var_name: &str, var_value: &str) -> bool {
    let output = Command::new("cmd")
        .args(&["/C", &format!("setx {} {}", var_name, var_value)])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                true
            } else {
                println!("错误: {}", String::from_utf8_lossy(&output.stderr));
                false
            }
        }
        Err(e) => {
            println!("执行命令失败: {}", e);
            false
        }
    }
}

fn set_env_var_unix(var_name: &str, var_value: &str) -> bool {
    // 在 Unix 系统上，我们建议用户手动添加到 shell 配置文件中
    println!("在 Linux/macOS 上永久设置环境变量:");
    println!("请将以下行添加到您的 shell 配置文件中:");
    println!("  export {}=\"{}\"", var_name, var_value);
    println!();
    println!("对于 bash，请添加到 ~/.bashrc 或 ~/.bash_profile");
    println!("对于 zsh，请添加到 ~/.zshrc");
    println!("对于 fish，请添加到 ~/.config/fish/config.fish");
    println!();
    println!("或者，您可以运行以下命令来自动添加(请先检查是否正确):");
    println!(
        "  echo 'export {}=\"{}\"' >> ~/.bashrc",
        var_name, var_value
    );
    true
}
