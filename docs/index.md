# Bevy 学习路线

「能运行一个窗口」只证明平台和依赖链暂时没有报错。学习 Bevy 的目标是把玩法压缩为可验证的状态、数据流与调度关系。

```{toctree}
:maxdepth: 1
:caption: 学习路径

roadmap
00-app-runtime/README
01-plugins-resources-systems/README
02-debug-visualization/README
stages/stage-00-observability/README
03-ecs-data-model/README
04-query-access-conflicts/README
capstone
contributing
```

## 当前基线

- Rust stable 与 Cargo
- Bevy `0.19.1`
- 每个章节目录都是可独立运行的 Cargo crate
- 每个章节的 README 同时呈现 Bevy 机制与对应的 Rust 设计说明

阶段实验插入在其对应阶段的最后一篇文章之后。站点发布时会一并复制每个目录的 Cargo 源码，因此 README 中的 `src/main.rs` 链接可直接打开发布版本源码。