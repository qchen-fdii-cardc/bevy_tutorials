# Bevy 教程 02：调试可视化：日志、Gizmos 与最小复现

「画面没报错」只说明你还没看见错误；它不是运行时正确性的证据。

## 省流版

- 调试要同时观察状态、行为和表现：结构化日志记录状态，Gizmos 标记空间关系，最小复现删掉无关变量。
- `DebugState` 是显式 Resource，统一保存调试开关和采样数据；按 `D` 改变它，按 `N` 生成一个可被日志和 Gizmos 同时观察的实体。
- `Query<Entity>` 可以提供实体数量这个粗粒度健康指标；它不能代替性能分析，但能立刻揭露「每帧都在生成实体」这类生命周期错误。
- `Commands` 的生成操作延迟到系统结束后应用。因此按 `N` 的同一帧日志仍可能显示旧计数，下一帧才会看见新实体。
- 系统根本没被 `add_systems` 注册时，日志、Gizmos 和状态变化会一起沉默。先检查调度，再怀疑算法。

## 运行方式

在本目录执行：

```powershell
cargo run
```

窗口默认显示绿色 X 轴、红色 Y 轴及两个黄色探针圆。按 `D` 隐藏或显示调试工具；按 `N` 生成新探针。终端每秒输出一次帧时间、实体数和探针数。

本章验证命令：

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

## 先问：你要证明什么

调试失败常常不是因为工具不够，而是问题没有被压缩成可证伪的判断。本章只验证三件事：系统是否运行，状态是否更新，空间中的实体是否符合预期。

| 要验证的判断 | 证据 | 本章工具 |
| --- | --- | --- |
| 系统正在被调度 | 每秒一次的结构化 `info!` 日志 | `report_runtime_state` |
| 输入改变了 World | `DebugState.enabled` 与探针计数变化 | 日志与 Resource |
| 实体坐标正确 | 圆环画在每个 `DebugMarker` 的 `Transform` 位置 | `Gizmos` |

> 调试输出不是写给「未来也许会看」的人。每一条输出都必须能推翻一个具体假设：系统没跑、Resource 没变、实体没生成，或坐标算错。

## 三条观察通道

### 结构化日志：观察运行时状态

[`src/main.rs`](src/main.rs) 中的 `report_runtime_state` 每帧累计 `Time::delta_secs()`，但只在秒数跨越整数时输出：

```rust
info!(
    elapsed_seconds = state.elapsed_seconds,
    frame_count = state.frame_count,
    frame_delta_milliseconds = time.delta_secs() * 1_000.0,
    entity_count = entities.iter().count(),
    marker_count = markers.iter().count(),
    "Observable runtime state"
);
```

字段比拼接字符串可靠。日志筛选器可以按 `marker_count`、`entity_count` 或事件名称检索，后续接入 tracing 时也不会把一整行文本重新拆开。

### Gizmos：观察空间关系

`draw_debug_gizmos` 从 `Query<&Transform, With<DebugMarker>>` 读取探针位置，并在每个位置画一个黄色圆。坐标轴让世界原点成为可见事实：绿色线是 X 轴，红色线是 Y 轴。

Gizmos 不保存玩法状态。它只读取现有状态并生成调试绘制命令。把碰撞半径、目标位置、移动向量画出来，远比盯着一个「角色好像有点偏」的屏幕更接近可复现证据。

### 最小复现：删除到只剩因果链

这个项目没有精灵、物理 crate、UI 框架或资产加载。按 `N`，输入系统查询当前探针数并通过 `Commands` 排队生成新实体；日志系统统计实体；Gizmos 系统绘制实体位置。这就是一条足够短的因果链：

```text
KeyN -> spawn_debug_marker -> Commands queue -> DebugMarker entity
     -> report_runtime_state log
     -> draw_debug_gizmos circle
```

Bug 仍能在这里复现，根因就在输入、状态、调度或坐标模型中。Bug 在这里消失，再逐个恢复外部依赖，而不是把整个游戏项目送进「玄学调参」的黑箱。

## `Commands` 延迟应用的观察陷阱

按 `N` 时，`spawn_debug_marker` 调用 `commands.spawn`。这不会立刻修改当前系统看到的 `Query`；Bevy 在适当的命令应用点将变更写进 World。因此本例把生成、报告和绘制串为 `.chain()`，让每帧因果顺序稳定，但探针新增仍在命令实际应用后才对后续帧可见。

这里得到的结论很具体：日志中暂时看不到新实体，并不自动证明输入失败。先确认 `Commands` 的延迟语义，再决定是否需要显式应用命令，或把后续观察放在正确的调度位置。

## 故障注入：定位「系统根本没有被调度」

执行下面实验，每次只改一处：

1. 从 `.add_systems(Update, ...)` 的元组中删除 `draw_debug_gizmos`，然后运行。日志仍每秒更新，按 `N` 后 `marker_count` 增加，屏幕上的黄色圆却冻结。这证明状态链还活着，表现系统没有调度。
2. 恢复绘制系统，再删除 `report_runtime_state`。圆环可以随按 `N` 增加，终端不再有运行状态。这次是日志通道被切断。
3. 将 `spawn_debug_marker` 从 `Update` 元组中删除。按 `N` 后既没有新圆，也没有新增计数；此时检查输入系统是否被注册，比检查键盘硬件更便宜。
4. 将 `setup` 错放入 `Update`。实体数会持续增加，黄色圆与相机会不断堆积。这是最小复现中最值钱的一类错误：生命周期归属错了，性能问题只是利息。

每个症状都对应不同层级。不要看到圆没出现就立刻重写 Gizmos；证据链先告诉你是输入、状态、调度还是表现断了。

## 本章练习

1. 为 `DebugMarker` 增加一个数值 ID，并让日志报告最后生成的 ID。该 ID 应存放在 Component 还是 Resource？说明理由。
2. 添加按 `Backspace` 删除最后一个探针的系统。用日志和 Gizmos 证明实体真正消失。
3. 新建一个每帧运行但只会记录一次的系统。先忘记把它传给 `add_systems`，观察完全沉默的症状；再注册它，确认日志只出现一次。
4. 故意把一个探针的 `Transform` 写成屏幕像素值。调整窗口大小后观察偏差，为下一阶段的坐标系问题留下最小复现。

## 下一章

下一篇进入 ECS 的数据模型：实体没有行为，Component 才是事实，System 只是对事实的变换规则。调试工具已经让 World 可见，接下来需要学会如何不把 World 建成一团不可查询的状态泥浆。

## 延伸阅读

- [Bevy `0.19.1`：Gizmos](https://docs.rs/bevy/0.19.1/bevy/gizmos/struct.Gizmos.html)
- [Bevy `0.19.1`：Commands](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.Commands.html)
- [Bevy 官方示例：2D Gizmos](https://bevy.org/examples/gizmos/2d-gizmos/)
- [Bevy 官方示例：ECS](https://bevy.org/examples/ecs-entity-component-system/)
- [Bevy 官方文档](https://bevy.org/learn/)