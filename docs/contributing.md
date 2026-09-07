# 文档与分支流程

`main` 是发布分支。GitHub Actions 只在推送到 `main` 后构建并部署 Pages，因此它必须保持可发布状态。

## 新增或修改教程

1. 从 `main` 创建主题分支，例如 `docs/chapter-05-commands` 或 `feature/stage-01-spore-simulation`。
2. 每个章节目录同时维护 `README.md` 与 `Rust_in_Bevy.md`，并运行该 crate 的 `cargo fmt --check`、Clippy、测试和示例。
3. 阶段完成后，在 `stages/stage-NN-[topic]/` 创建独立验收实验，使用相同的四文件结构。
4. 在拉取请求中审阅源码、文档链接和构建结果；合并到 `main` 后，Pages 工作流自动发布。

本地构建文档站：

```powershell
docs\.venv\Scripts\python.exe docs\build.py
```

生成中文 PDF：

```powershell
docs\.venv\Scripts\python.exe docs\build.py --pdf
```

PDF 输出在 `docs/_build/latex/`，HTML 输出仍在 `docs/_build/html/`。PDF 构建使用已安装的 XeLaTeX 和系统字体 `Microsoft YaHei`。

生成的网站在 `docs/_build/html/index.html`。发布后，源代码也位于站点根目录的对应 Cargo 目录，可从教程页面的源码链接直接打开。