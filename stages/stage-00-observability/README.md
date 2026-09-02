# 阶段 0 实验：可观测的 Bevy 运行时

「窗口弹出来了」不构成运行时正确性的证据。你必须看见帧时间、实体数量和世界坐标，才有资格判断系统到底在做什么。

## 省流版

- 这是阶段 0 的集成实验，汇合文章 00 的 App、文章 01 的 Plugin/Resource 边界和文章 02 的日志、Gizmos 与最小复现方法。
- 窗口左上角实时显示帧时间、近似 FPS、实体数和探针数；按 `D` 切换覆盖层，按 `N` 生成探针。
- Gizmos 始终绘制绿色 X 轴、红色 Y 轴和黄色探针圆。覆盖层隐藏不等于调试系统停止运行。
- 结构化日志每秒报告指标，验证采样、实体查询和 System 调度链仍在运行。

## 运行方式

在本目录执行：

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

启动后依次验证：

1. 观察左上角覆盖层的 `frame`、`fps`、`entities` 和 `markers`。
2. 按 `N`，黄色探针增加，覆盖层和终端日志的计数在下一帧更新。
3. 按 `D`，文本覆盖层消失或恢复，坐标轴与探针仍继续绘制。
4. 观察终端每秒的 `Observable runtime state` 和 `Overlay metrics refreshed` 记录。

## 问题模型

| 状态 | 归属 | 写入者 | 观察者 |
| --- | --- | --- | --- |
| 时间、帧计数、FPS | `RuntimeStats` Resource | `collect_runtime_stats` | UI、日志 |
| 探针位置 | `Transform` Component | `spawn_debug_marker` | Gizmos、UI、日志 |
| 覆盖层可见性 | UI 实体的 `Visibility` Component | `toggle_debug_overlay` | 渲染器 |

调试信号必须来自真实 World 状态。把「实体数」维护为一个随手递增的全局变量会在 despawn、加载和失败路径上立刻失真；直接查询 `Query<Entity>` 才是当前 World 的事实。

## 故障注入

1. 从 `Update` 移除 `collect_runtime_stats`：覆盖层中的时间和 FPS 冻结，日志也不再按秒产生。根因是采样系统没有被调度。
2. 从 `Update` 移除 `update_overlay_text`：终端日志仍刷新，屏幕指标冻结。状态存在，表现映射断了。
3. 将 `draw_debug_gizmos` 从 `Update` 移除：覆盖层和日志仍显示探针数，世界中不再有坐标轴或圆环。渲染调试层失效。
4. 将 `setup` 错放到 `Update`：相机、UI 和初始探针每帧增长，实体数迅速暴涨。生命周期归属错误会先表现为观测信号异常。

## 验收

- 可解释 `DefaultPlugins` 为该实验提供窗口、输入、时间、渲染、UI 和 Gizmos。
- 能自行向 `RuntimeStats` 添加一个指标，并同时更新 UI 和结构化日志。
- 能通过「日志是否变化、覆盖层是否变化、Gizmos 是否变化」定位一个未注册的 System。

## 依赖配置

```toml
bevy = { version = "0.19.1", default-features = false, features = ["default_app", "default_platform", "2d_bevy_render", "ui"] }
```

前三个 feature 提供 App、平台事件、2D 相机和 Gizmos；`ui` 提供 `Node`、`Text`、`TextFont` 和 `TextColor`。此实验刻意排除 3D PBR、GLTF 和音频。根目录的 `.cargo/config.toml` 将构建工件放进共享 `target`，供相邻章节复用。

## 延伸阅读

- [Bevy `0.19.1`：UI](https://docs.rs/bevy/0.19.1/bevy/ui/)
- [Bevy `0.19.1`：Text](https://docs.rs/bevy/0.19.1/bevy/text/)
- [Bevy `0.19.1`：Gizmos](https://docs.rs/bevy/0.19.1/bevy/gizmos/struct.Gizmos.html)
- [Bevy 官方示例索引](https://bevy.org/examples/)