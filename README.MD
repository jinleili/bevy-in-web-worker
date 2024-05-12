# Bevy-in-Web-Worker

## 安装 Rust 环境

#### 在 Linux 或 macOS 上安装

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

#### 在 Windows 上安装

[https://course.rs/first-try/installation.html#在-windows-上安装-rustup](https://course.rs/first-try/installation.html#在-windows-上安装-rustup)

## 安装 VSCode 插件（查看源码时才需要）

- rust-analyzer

## 浏览器端运行

```sh
# 编译 WASM：
# 添加 Rust WebAssembly target
rustup target add wasm32-unknown-unknown
# 安装 wasm-bindgen 命令行工具
cargo install -f wasm-bindgen-cli --version 0.2.92

# 运行：
# 先安装 http server
cargo install basic-http-server
# 然后使用下边的脚本运行
sh ./run-wasm.sh

# 构建 release 包
sh ./build-wasm.sh
```
