# Bevy 学习路线

「能运行一个窗口」只证明平台和依赖链暂时没有报错。学习 Bevy 的目标是把玩法压缩为可验证的状态、数据流与调度关系。

```{toctree}
:maxdepth: 2
:caption: 路线图

roadmap
bevy/index
rust/index
stages/index
contributing
```

## 当前基线

- Rust stable 与 Cargo
- Bevy `0.19.1`
- 每个章节都是可独立运行的 Cargo crate
- 每个章节都有对应的 Rust 语言构造与设计技术说明

站点发布时会一并复制所有章节和阶段实验的 Cargo 源码。教程页中的源码链接可直接打开发布版本的 `src/main.rs`、`Cargo.toml` 和配套 Markdown。