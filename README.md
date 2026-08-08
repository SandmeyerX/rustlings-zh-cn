# Rustlings-zh-cn 🦀❤️

![Tracking Status](https://img.shields.io/badge/track-当前已为最新版本-green)

>[!NOTE]
>新版 Rustlings 的 README 文件移除了以下内容，取而代之的是网站链接：[Rustlings](https://rustlings.rust-lang.org) 🦀
>这里为了直观地展示安装步骤，保留了原版内容的翻译

欢迎使用 Rustlings 练习题简体中文版 😃
这个项目包含一些小练习，来帮助你习惯阅读和编写惯用的 Rust 代码。
也包括如何阅读编译器提示并据此修改代码！

建议在做 Rustlings 练习的同时阅读 [《Rust 程序设计语言》（The Rust Book）](https://kaisery.github.io/trpl-zh-cn/)（社区中文翻译，原文：[The Rust Book](https://doc.rust-lang.org/book/)），这是学习 Rust 最全面的资源📚️

[Rust By Example](https://rustwiki.org/zh-CN/rust-by-example/)（社区中文翻译，原文：[Rust By Example](https://doc.rust-lang.org/rust-by-example/)）是另一个推荐的资源，可能会对你有帮助。
它包含了与 Rustlings 类似的代码示例和练习，但适合在线上使用。

✨ 如有任何问题，请在 [issues 中联系我们](https://github.com/SandmeyerX/rustlings-zh-cn/issues)！

## 入门指南

### 安装 Rust
在安装 Rustlings 之前，需要安装 **最新版本的 Rust**。
访问 [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) 以获取安装 Rust 的更多说明。
这同时将安装 Rust 的包/项目管理器 _Cargo_。

> 🐧 如果使用 Linux，请确保已安装 `gcc`（用于链接器）。
>
> Deb：`sudo apt install gcc`。
> Dnf：`sudo dnf install gcc`。

> 🍎 如果使用 macOS，请通过运行 `xcode-select --install` 确保已安装 Xcode 及其开发工具。

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
安装 Rustlings 后，运行以下命令将简体中文版练习题下载到本地：

```bash
git clone https://github.com/SandmeyerX/rustlings-zh-cn.git --depth 1
```

或者[点击这里下载最新版压缩包](https://github.com/SandmeyerX/rustlings-zh-cn/releases/latest/download/release.zip)

### 开始练习
练习题下载完成后，在解压后的练习题根目录下运行以下命令:
```bash
rustlings
```

<details>
<summary><strong>如果提示找不到命令<code>rustlings</code>（command cannot be found）…</strong>（<em>点击展开</em>）</summary>

可能使用的是 Linux 并通过包管理器安装了 Rust。
Cargo 将二进制文件安装到 `~/.cargo/bin` 目录。
遗憾的是，包管理器通常不会将 `~/.cargo/bin` 添加到您的 `PATH` 环境变量中。

解决方案是…

- 手动将 `~/.cargo/bin` 添加到 `PATH`
- 或者从包管理器中卸载 Rust，并使用官方的 `rustup` 方式安装：https://www.rust-lang.org/tools/install

</details>
