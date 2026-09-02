# Bevy 学习路线最终项目

这个仓库根目录是路线图最终 2D 垂直切片的 Cargo 项目。它将随着后续阶段加入移动、碰撞、敌人、UI、音效、存档、场景状态和发布配置。

课程章节与阶段实践位于 [`docs/`](docs/)，每个目录都是独立、可运行、可验证的最小实验。根项目不复制这些章节代码；它只吸收已经验收的设计，避免把学习过程变成一个难以回滚的巨型 `main.rs`。

## 当前状态

目前根项目仅启动 Bevy `0.19.1` 的最小 2D 运行时。运行：

```powershell
cargo run
```

文档站本地构建：

```powershell
docs\.venv\Scripts\python.exe docs\build.py
```

## 项目与源码

<a href="https://github.com/qchen-fdii-cardc/bevy_tutorials">GitHub 仓库</a> · <a href="Cargo.toml">查看 <code>Cargo.toml</code></a> · <a href="src/main.rs">查看 <code>src/main.rs</code></a> · <a href="https://www.windtunnel.cn/bevy_tutorials/">在线文档</a>

## 目录边界

- `docs/00-*`、`docs/01-*`：按文章编号组织的学习章节。
- `docs/stages/stage-NN-*`：每个阶段的独立验收实验，插在对应阶段的最后一篇章节之后。
- `src/`：最终 2D 垂直切片。只在章节机制经过独立实验验证后加入。