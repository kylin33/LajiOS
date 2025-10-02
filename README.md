# LajiOS
a Rust simple  operating system

## 环境搭建

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

可以手动下载https://sh.rustup.rs中rustup-init.sh 文件，然后修改RUSTUP_UPDATE_ROOT 变量改为 https://mirrors.ustc.edu.cn/rust-static/rustup 进行安装，中间会有提示菜单，选择默认即可。

### 2. 安装 rust nightly

```bash
rustup override add nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

### 3. 安装 qemu

```bash
sudo apt install qemu-system-x86 qemu-kvm gcc
```


### 4. 安装bootimage

需要引导程序将负责初始化 CPU 并加载我们的内核。
```bash
cargo install bootimage
rustup component add llvm-tools-preview
```

### 5. 编译与运行

```bash
cargo build
cargo run
```
