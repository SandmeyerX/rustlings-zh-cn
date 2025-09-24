# Rustlings-zh-cn 🦀❤️

![Tracking Status](https://img.shields.io/badge/track-版本落后待更新-red)

欢迎使用 Rustlings 练习题 简体中文版 😃
这个项目包含一些小练习，旨在帮助您习惯阅读和编写惯用的 Rust 代码。
这其中也包括阅读和参照编译器的信息！

建议在做 Rustlings 练习的同时阅读 [Rust官方书籍(The Rust Book)](https://doc.rust-lang.org/book/)，这是学习 Rust 最全面的资源📚️

[Rust By Example](https://doc.rust-lang.org/rust-by-example/) 是另一个推荐的资源，您可能会发现它很有帮助。
它包含与 Rustlings 类似的代码示例和练习，但需要在线上使用。

✨ 如果您在使用中遇到了任何问题，请在[此处](https://github.com/SandmeyerX/rustlings-zh-cn/issues)与我们联系！

## 入门指南

### 安装 Rust
在安装 Rustlings 之前，您需要安装 **最新版本的 Rust**。
访问 [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 以获取安装 Rust 的更多说明。
这也将安装 Rust 的包/项目管理器 _Cargo_。

> 🐧 如果您使用 Linux，确保已安装 `gcc`（用于链接器）。
>
> Deb：`sudo apt install gcc`。
> Dnf：`sudo dnf install gcc`。

> 🍎 如果您使用 MacOS，通过运行 `xcode-select --install` 确保已安装 Xcode 及其开发工具。

### 安装 Rustlings
以下命令将下载并编译 Rustlings：

```bash
cargo install rustlings
```

<details>
<summary><strong>如果安装失败…</strong>（<em>点击展开</em>）</summary>

- 通过运行 `rustup update` 确保您拥有最新的 Rust 版本
- 尝试添加 `--locked` 标志：`cargo install rustlings --locked`
- 否则，请 [报告问题](https://github.com/rust-lang/rustlings/issues/new)

</details>

### 下载练习题
安装 Rustlings 后，运行以下命令将 `rustlings练习-简体中文版` 下载到本地:

```bash
git clone https://github.com/SandmeyerX/rustlings-zh-cn.git
```

或者[点击链接](https://github.com/SandmeyerX/rustlings-zh-cn/archive/refs/heads/main.zip)下载项目最新压缩包

### 开始练习
练习题下载完成后，在解压后的 `rustlings-zh-cn` 目录下运行以下命令:
```bash
rustlings
```

<details>
<summary><strong>如果提示找不到命令<code>rustlings</code>(command cannot be found)…</strong>（<em>点击展开</em>）</summary>

您可能使用的是 Linux 并通过包管理器安装了 Rust。
Cargo 将二进制文件安装到 `~/.cargo/bin` 目录。
遗憾的是，包管理器通常不会将 `~/.cargo/bin` 添加到您的 `PATH` 环境变量中。

解决方案是…

- 手动将 `~/.cargo/bin` 添加到 `PATH`
- 或者从包管理器中卸载 Rust，并使用官方的 `rustup` 方式安装：https://www.rust-lang.org/tools/install

</details>
