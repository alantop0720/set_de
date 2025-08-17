# set_de - 环境变量设置工具

一个用于设置和管理Python开发环境相关环境变量的命令行工具。

## 功能介绍

该工具主要用于简化Python开发环境的配置过程，提供了以下主要功能：

1. 读取和设置 `UV_PYTHON_INSTALL_MIRROR` 环境变量
   - 该环境变量用于配置 Python 解释器下载镜像源
   - 默认使用镜像源: `https://ghfast.top/https://github.com/astral-sh/python-build-standalone/releases/download`

2. 读取和设置 `UV_DEFAULT_INDEX` 环境变量
   - 该环境变量用于配置 Python 包索引源
   - 默认使用镜像源: `https://pypi.tuna.tsinghua.edu.cn/simple`

3. 读取和写入 Windows 系统中的 pip.ini 配置文件
   - 自动配置 pip 使用清华大学 PyPI 镜像源
   - 配置内容包括索引URL和可信主机

4. 读取和写入 Windows 系统中的 uv.toml 配置文件
   - 自动配置 uv 使用阿里云或清华大学 PyPI 镜像源
   - 配置内容包括索引URL和默认设置

## 特性

- 跨平台支持：兼容 Windows、Linux 和 macOS 操作系统
- 永久设置：可以将环境变量永久保存到系统中
- 用户友好：提供清晰的菜单界面和操作提示
- 自动配置：自动创建所需的目录和配置文件

## 使用方法

运行程序后，根据菜单提示选择相应的操作：

```
==============================
     环境变量设置程序
==============================
1. 读取 UV_PYTHON_INSTALL_MIRROR 环境变量
2. 永久设置 UV_PYTHON_INSTALL_MIRROR 环境变量
3. 读取 UV_DEFAULT_INDEX 环境变量
4. 永久设置 UV_DEFAULT_INDEX 环境变量
5. 读取 pip.ini 文件内容
6. 写入 pip.ini 文件内容
7. 读取 uv.toml 文件内容
8. 写入 uv 索引配置到 uv.toml 文件
9. 退出
==============================
请选择操作:
```

## 注意事项

1. 在 Windows 系统上设置环境变量可能需要管理员权限
2. 新设置的环境变量仅在新打开的终端/命令行窗口中生效
3. 在 Linux/macOS 系统上，程序会提示用户手动将环境变量添加到 shell 配置文件中
4. 对于 pip.ini 配置文件，程序会自动在用户目录下创建 pip 文件夹（如果不存在）
5. 对于 uv.toml 配置文件，程序会自动在用户目录下创建相应文件夹（如果不存在）

## 构建和运行

```bash
# 构建项目
cargo build

# 运行项目
cargo run

# 发布构建
cargo build --release
```