# Bevy 教程 00：`App` 如何开始运行

「`App::new()` 后面接几行链式调用」不叫理解 Bevy；那只是在 Rust 里背下了一段启动咒语。

## 省流版

- `App` 保存游戏运行时的结构：插件、世界、资源、系统与调度表都在这里汇合。
- `DefaultPlugins` 提供窗口、渲染、输入、时间等默认能力；删掉它，示例没有理由凭空获得窗口和 2D 相机。
- `Startup` 适合执行一次性初始化；`Update` 每帧执行。把两者混在一起，生命周期就开始发臭。
- `Resource` 存放跨实体共享的状态。这里的 `RuntimeStats` 记录运行时间和帧数，并每秒输出一次日志。
- Gizmos 是低成本的可观测性工具。先看见坐标轴和跳动圆环，再相信系统真的在调度；肉眼观察不是单元测试，但比闭眼敲 API 强得多。（确信

## 运行方式

在本目录执行：

```powershell
cargo run
```

会出现一个窗口：绿色横轴、红色纵轴和一个随时间呼吸的白色圆环。终端会每秒输出一次 `RuntimeStats`。关闭窗口即可结束程序。

本章的验证命令：

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check
```

## 依赖配置

本章没有使用 `bevy = "0.19.1"` 的完整默认 feature 集，而是固定为：

```toml
bevy = { version = "0.19.1", default-features = false, features = ["default_app", "default_platform", "2d_bevy_render"] }
```

`default_app` 提供 `App`、日志、资产与状态等基础运行时；`default_platform` 提供窗口、键盘和平台事件循环；`2d_bevy_render` 提供 `Camera2d`、2D 渲染与 Gizmos。本章不使用 3D PBR、GLTF、UI、音频、场景序列化或 picking，因此不编译它们。仓库根目录的 `.cargo/config.toml` 将所有独立章节的构建工件放在共享 `target` 目录，后续章节复用相同 feature 集的依赖缓存。

## 先把运行时拆开

任何可运行的 Bevy 程序都可以先压缩成下面这张图：

```text
App
 |- Plugins: 提供窗口、渲染、时间等基础能力
 |- World: 保存 Entity、Component 和 Resource
 |- Schedules: 决定 System 何时执行
 `- Systems: 读取并修改 World 中的状态
```

`App` 没有神秘魔法。它本质上是在组装运行时：先注册能力和数据，再告诉调度器「哪些函数应在哪个阶段运行」，最后进入事件循环。窗口事件到来后，Bevy 推进时间、收集输入、执行系统、准备渲染并提交画面；这是一条数据流，不是一台藏在宏背后的黑箱。

> 学习 Bevy 的第一条纪律：**每次新增 API 调用，都要回答它是在添加能力、添加状态，还是添加状态变换规则。**
>
> `add_plugins` 添加能力，`insert_resource` 添加状态，`add_systems` 添加规则。连这个分类都无法完成时，教程进度条只是认知负荷的装饰品。

## 代码逐段解剖

以下内容以本目录的 [`src/main.rs`](src/main.rs) 为准，适用于 Bevy `0.19.1`。

```rust
App::new()
    .insert_resource(RuntimeStats::default())
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, (advance_runtime, draw_debug_axes).chain())
    .run();
```

这五行分别在做五件可验证的事：

1. `App::new()` 创建空的应用容器。
2. `insert_resource` 将唯一的 `RuntimeStats` 放进 World。
3. `DefaultPlugins` 注册窗口、渲染、时间和输入等默认插件集合。
4. `setup` 被安排到 `Startup`，只初始化一次；`advance_runtime` 与 `draw_debug_axes` 被安排到每帧运行的 `Update`。
5. `run` 取得控制权并进入平台事件循环；它之后的普通 Rust 语句不会执行。

`RuntimeStats` 标注 `#[derive(Resource)]`。这个标记不是装饰，它告诉 Bevy：该类型以单例形式存于 World，可由系统通过 `Res<T>` 读取、通过 `ResMut<T>` 修改。时间 `Time` 同样是一个由引擎维护的 Resource，因此 `advance_runtime` 的签名已经把依赖关系写得很直白：读取时间，写入统计数据。

```rust
fn advance_runtime(time: Res<Time>, mut stats: ResMut<RuntimeStats>)
```

这里有一个需要立刻建立的边界：`Time` 表示真实帧间隔，适合视觉动画、UI 和输入采样。需要确定性规则的移动、碰撞和战斗逻辑，应在之后的 `FixedUpdate` 学习；把物理规则绑死在屏幕刷新频率上，30 FPS 与 144 FPS 用户会玩到两个不同游戏。

## `Startup` 和 `Update` 不是两个随便挑的标签

`setup` 生成 `Camera2d`，所以它属于 `Startup`。每帧创建一个相机等于不断向 World 塞入新的视角实体，常见症状是画面、性能和调试信息一起失控。一次性创建资源、相机、初始地图和初始 UI，通常从 `Startup` 开始检查。

`advance_runtime` 累积 `delta_secs()`，`draw_debug_axes` 根据已累积时间计算圆环半径，因此两者必须按此顺序执行：

```rust
(advance_runtime, draw_debug_axes).chain()
```

`chain()` 将两个系统串成明确顺序。它没有让代码「更优雅」，它是在声明真实的数据依赖：先写 `RuntimeStats`，再读 `RuntimeStats`。当系统关系还没有数据依赖时，不要迷信全局排序；让 Bevy 并行调度独立系统，才是 ECS 的正常打开方式。

## 故障注入：删掉一行，看看你到底懂了什么

请依次做下面三个实验，每次只改一处，再运行 `cargo run`：

1. 删除 `.add_plugins(DefaultPlugins)`：程序失去窗口、时间与渲染相关能力，编译或运行行为会立刻揭露这些能力原先从哪里来。
2. 将 `setup` 放到 `Update`：每帧生成一台 `Camera2d`。用日志或实体检查工具确认实体数量持续增长，然后将它放回 `Startup`。
3. 删除 `.chain()`：圆环仍可能看起来正常，因为 Bevy 可自行安排读取与写入冲突；这正是危险之处。当前逻辑存在顺序依赖，文章不能靠「大部分时间看起来没问题」给错误设计开绿灯。

第三个实验的结论很冷：画面偶然正确不构成架构正确。调度问题一旦混入复杂玩法，会以偶发、平台相关、难以复现的方式收债。

## 本章练习

将 `RuntimeStats` 扩展为一个真正可观察的小实验：

- 记录最近一秒的帧数，并计算近似 FPS。
- 在圆环上增加一个沿 X 轴往复运动的点；点的位置必须由 Resource 中的时间推导，不能偷塞进全局 `static`。
- 新增 `Paused` Resource，并用键盘切换它；暂停时统计数据不再累积，Gizmos 仍绘制坐标轴。

练习完成后，回答一个问题：暂停状态属于单个实体，还是属于整个游戏世界？如果答案没有落到 Resource、Component 或 Event 的具体选择上，说明状态归属仍是模糊的。

## 下一章

下一篇将处理插件、资源与系统的组合边界：如何把一个功能做成可插拔模块，又不把全局状态散落成难以追踪的隐式依赖。到那时，`App` 才会从启动代码变成可维护项目的装配点。

## 延伸阅读

- [Bevy 官方文档：Learn](https://bevy.org/learn/)
- [Bevy `0.19.1`：`App`](https://docs.rs/bevy/0.19.1/bevy/prelude/struct.App.html)
- [Bevy `0.19.1`：`Resource`](https://docs.rs/bevy/0.19.1/bevy/ecs/resource/trait.Resource.html)
- [Bevy 官方示例：Gizmos](https://bevy.org/examples/gizmos/2d-gizmos/)
- [Bevy 迁移指南](https://bevy.org/learn/migration-guides/)