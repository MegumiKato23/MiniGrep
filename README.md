# MiniGrep

MiniGrep 是一个使用 Rust 编写的高性能文件内容搜索工具。它支持并行搜索，可以快速在大型代码库或文本文件中查找指定内容。

## 特性

- 异步并行搜索，提供更快的搜索速度
- 递归搜索目录
- 支持命令行参数配置
- 友好的输出格式，包含文件路径和行号
- 使用 tokio 运行时提供高性能异步 I/O

## 安装

确保你已经安装了 Rust 工具链，然后执行：

```bash
git clone https://github.com/MegumiKato23/MiniGrep.git
cd MiniGrep
cargo install --path .
```

## 使用方法
基本用法：
```bash
minigrep <搜索文本> [搜索目录]
```
参数说明：
- <搜索文本> : 必填，要搜索的文本内容
- [搜索目录] : 可选，要搜索的目录路径，默认为当前目录