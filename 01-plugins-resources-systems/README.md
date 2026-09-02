# Bevy 教程 01：插件、资源与系统：先把运行时的骨架搭出来

「把系统函数散落在 `main.rs`」不会让项目保持简单；它只会让每个新功能都变成一次全局搜索。

## 省流版

- `Plugin` 是功能模块的装配入口：它集中注册 Resource、System、事件和子插件，主程序只决定模块组合与顺序。
- `Resource` 保存跨实体共享的唯一状态。本章的 `DebugOverlay` 保存开关，`Heartbeat` 保存运行时钟；它们的所有权和读写边界都由系统签名暴露。
- `System` 应按数据依赖连接：输入系统先改 `DebugOverlay`，时间系统再改 `Heartbeat`，最后绘制系统只读取它们。
- `DefaultPlugins` 提供窗口、输入、时间和 Gizmos 的基础能力；自定义插件只拥有自己的功能状态，别顺手把整个游戏塞进去。
- 按 `D` 切换调试覆盖层。看得见状态变化、看得到日志，才谈得上模块边界。（确信

## 运行方式

在本目录执行：

```powershell
cargo run
```

窗口启动后按 `D`：青色呼吸圆和黄色横线出现或消失，终端会输出开关状态及每秒一次的运行日志。

本章验证命令：

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

## 主函数只做装配

本章的主函数刻意短到近乎无聊：

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RuntimeDiagnosticsPlugin)
    .run();
```

它表达的意思足够具体：先加入 Bevy 的基础运行时，再加入一个「运行时诊断」功能模块，随后进入事件循环。主函数不该知道按键如何切换覆盖层，也不该知道圆环半径如何计算。那些都属于模块内部的规则。

> 判断一段逻辑是否应该进 `Plugin::build`，可以问一句：**删掉这个功能后，这些 Resource、System 和初始化实体是否应该一起消失？**
>
> 答案为「是」，它们就应由同一个插件注册。功能卸载时留下半截全局状态，是大型 Bevy 项目里最常见的隐式债务。

## `Plugin::build` 是模块边界

[`src/main.rs`](src/main.rs) 中的核心是：

```rust
impl Plugin for RuntimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugOverlay>()
            .init_resource::<Heartbeat>()
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (toggle_debug_overlay, advance_heartbeat, draw_debug_overlay).chain(),
            );
    }
}
```

`init_resource::<T>()` 在 World 还没有 `T` 时插入其 `Default` 值；已存在时不覆盖。这一点决定了插件能否被宿主配置：应用可以在加入插件前通过 `insert_resource` 提供自定义值，插件不会粗暴地重置它。

这里的两个 Resource 具有不同职责：

| 类型 | 保存的状态 | 写入者 | 读取者 |
| --- | --- | --- | --- |
| `DebugOverlay` | 调试图层是否可见 | `toggle_debug_overlay` | `draw_debug_overlay` |
| `Heartbeat` | 累积运行时间与日志节流 | `advance_heartbeat` | `draw_debug_overlay` |

这张表比「这个变量放哪儿比较顺手」可靠得多。数据归属先清楚，系统签名和调度关系才会自然浮出来。

## 系统参数就是依赖声明

```rust
fn toggle_debug_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut overlay: ResMut<DebugOverlay>,
)
```

这个签名明确读取键盘输入、修改覆盖层状态。Bevy 据此检查同一调度中的冲突，并将无冲突系统并行执行。ECS 的重点从来不在把函数塞进某个宏；重点在让读写集合可见。

本例使用 `.chain()`，顺序是：输入切换、推进时钟、绘制覆盖层。前两个系统彼此没有数据依赖，理论上可以并行；这里仍显式串联，是为了让教学实验每一帧的因果链固定。实际项目中应只在确有依赖时排序，过度串行化会把 ECS 写回单线程脚本语言。

## 故障注入：让模块边界露馅

每次只改一处并运行 `cargo run`，观察日志和画面：

1. 删除 `.add_plugins(RuntimeDiagnosticsPlugin)`：窗口仍存在，因为 `DefaultPlugins` 还在；按 `D` 不再产生任何效果。由此可见窗口能力和诊断功能属于不同插件。
2. 注释掉 `.init_resource::<DebugOverlay>()`：系统声明依赖一个不存在的 Resource，运行时会报出明确错误。Resource 没有注册，系统就没有可操作的状态。
3. 把 `setup_camera` 从 `Startup` 移到 `Update`：每帧创建一个相机实体。这个错误的根因是生命周期归属错误，不是「引擎性能不行」。将它放回 `Startup`。
4. 在 `draw_debug_overlay` 中移除 `if !overlay.enabled { return; }`：按 `D` 的日志仍会变化，视觉结果却不变。状态被正确更新，状态到表现层的映射断了；这正是调试时应分层检查的证据链。

## 本章练习

1. 新建 `OverlayColor` Resource，让覆盖层颜色可配置；主函数在加入插件前提供一个非默认颜色。
2. 将 `setup_camera` 移到单独的 `CameraPlugin`。讨论它是否仍应该属于 `RuntimeDiagnosticsPlugin`：删除诊断功能后，应用是否仍需要相机？
3. 去掉 `.chain()`，再为 `draw_debug_overlay` 增加一个读取 `Heartbeat` 后才成立的条件。用 `.after(advance_heartbeat)` 只表达那一条真实依赖。

练习的验收标准很简单：你能说清每个 Resource 的默认值由谁提供，每个 System 的读写集合是什么，以及移除一个插件后 World 中应少掉哪些事实。

## 下一章

下一篇进入调试可视化：日志、Gizmos 与最小复现。调试 UI 只是手段；目标是让每个状态判断都能被观察、反驳和定位。

## 延伸阅读

- [Bevy `0.19.1`：`Plugin`](https://docs.rs/bevy/0.19.1/bevy/app/trait.Plugin.html)
- [Bevy `0.19.1`：`App::init_resource`](https://docs.rs/bevy/0.19.1/bevy/app/struct.App.html#method.init_resource)
- [Bevy `0.19.1`：`Resource`](https://docs.rs/bevy/0.19.1/bevy/ecs/resource/trait.Resource.html)
- [Bevy 官方示例：Gizmos](https://bevy.org/examples/gizmos/2d-gizmos/)
- [Bevy 官方示例索引](https://bevy.org/examples/)