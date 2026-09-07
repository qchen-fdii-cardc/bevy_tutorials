# 最终 2D 游戏：Arena 生存垂直切片

「能启动窗口」只是工程的起点。这个根目录项目把前面章节和阶段实践验证过的机制组合成一个可玩的 2D Arena 生存原型。

## 项目与源码

[GitHub 仓库](https://github.com/qchen-fdii-cardc/bevy_tutorials) · [在线文档](https://www.windtunnel.cn/bevy_tutorials/)

<details>
<summary>展开查看 <code>Cargo.toml</code></summary>

```{literalinclude} ../Cargo.toml
:language: toml
:caption: Cargo.toml
:linenos:
```
</details>

<details>
<summary>展开查看 <code>src/main.rs</code></summary>

```{literalinclude} ../src/main.rs
:language: rust
:caption: src/main.rs
:linenos:
```
</details>



根目录的 Cargo 项目是路线图的最终宿主。章节和阶段实验留在 `docs/`，这里只接收已经能够独立解释、测试和运行的机制。

## 游戏与玩法

这是一个俯视角 2D Arena 生存游戏。玩家在有限的矩形竞技场中移动，收集晶体、躲避敌人，并尽可能延长生存时间。

### 基本流程

1. 启动游戏后进入菜单，按 `Enter` 开始。
2. 使用 `WASD` 移动玩家。
3. 收集场地上的晶体，分数增加，晶体会在新的随机位置重新生成。
4. 敌人会追踪玩家，并通过分离力避免彼此重合；每个敌人还有轻微独立的随机游走，因此队伍会形成带有扰动的围剿，而不是一条僵硬的直线。
5. 敌人接触玩家会造成伤害并被重新生成；生命值归零后进入 Game Over。
6. 在 Game Over 状态按 `Enter` 重开，按 `Esc` 返回菜单。

HUD 显示分数、生命值和波次。菜单和游戏状态都由键盘操作驱动，游戏规则本身保留在 ECS System 中，渲染对象只承担表现职责。

## 如何运行

在仓库根目录执行：

```powershell
cargo run
```

发布构建：

```powershell
cargo build --release
```

本项目会在运行时生成最小的 PNG 和 WAV 资源到 `assets/generated/`，因此实验不依赖外部美术包即可启动。完整验证命令：

```powershell
cargo fmt --check
cargo check
cargo clippy --all-targets -- -D warnings
cargo test
```

## 实现方式

### `App`、状态与调度

入口通过 `App::new()` 注册 `DefaultPlugins`、`GameState`、`Settings` 和 `GameSession`，然后把逻辑分配到三个生命周期层：

- `Startup`：创建相机、背景、资源句柄和 HUD，并准备菜单实体。
- `Update`：处理菜单输入、Game Over 输入、HUD 文本和相机跟随。
- `FixedUpdate`：执行移动、敌人 AI、拾取、伤害、波次生成和边界约束。

`GameState` 包含 `Menu`、`Playing` 和 `GameOver`。`OnEnter(GameState::Playing)` 重置会话状态并重新生成玩家、敌人和晶体，避免重开游戏时把上一局实体残留到 World 中。

### ECS 数据模型

核心状态拆成 Resource 和 Component：

- `GameSession`：分数、生命值、波次和计时器等共享会话状态。
- `Settings`：音量设置等持久化配置。
- `GameAssets`：图片和音效的资源句柄。
- `Player`、`Enemy`、`Crystal`：实体分类标记。
- `Velocity`：移动速度事实。
- `Collider`：可参与碰撞规则的标记。
- `FlockWander`：敌人独立的随机游走相位和转向速率。

`Entity` 只提供身份，规则通过 Query 选择需要的事实。玩家、敌人和晶体没有被塞进一个万能对象；这使移动、拾取、伤害和表现可以分别测试和替换。

### 敌人 AI：追踪、分离与随机游走

每个固定时间步，敌人 AI 会完成三段计算：

1. 读取所有敌人的位置快照。
2. 对当前敌人周围 `46` 像素内的邻居计算分离向量，距离越近，排斥力越强。
3. 将玩家追踪方向、分离方向和 `FlockWander` 的平滑随机方向混合，再以 `ENEMY_SPEED` 更新位置。

代码使用 `ParamSet` 分开读取位置和写入敌人 Transform，避免在同一 System 内制造 ECS 借用冲突。`Without<Player>` 则向 Bevy 明确证明敌人写查询与玩家读查询不重叠。

这个设计形成一个可调的行为模型：提高分离权重会扩大敌人间距，提高游走权重会增加队形扰动，提高追踪速度会让围剿更直接。参数变化应通过游戏体验和测量共同决定，不能把一个看起来「聪明」的数字伪装成唯一正确答案。

### 交互、音频与资源

- 晶体拾取通过玩家与晶体 Transform 的距离判断触发。
- 敌人伤害通过玩家与敌人的距离判断触发。
- 音效使用 Bevy `AudioPlayer` 和 `PlaybackSettings::DESPAWN`，播放结束后自动清理实体。
- `AssetServer` 管理图片和 WAV 资源句柄。
- `ensure_generated_assets()` 生成最小可运行的圆形 PNG 和正弦波 WAV，降低实验对外部资源的依赖。

### UI 与配置

HUD 使用 Bevy UI 的 `Node` 和 `Text` Component。`update_hud` 从 `GameSession` 派生分数、生命值和波次文本，界面不会维护第二份游戏状态。

设置通过简单的 `settings.txt` 保存音量字段。它目前是一个有意保持克制的持久化边界：先让配置读写可观察，再在后续阶段替换为更完整的序列化方案。

## 与前面章节的对应关系

这个页面不是一份孤立的成品说明。每个实现选择都可以沿路线回溯到更小的实验：

### 阶段 0：运行时与可观测性

- 机制：`App`、`Startup`、`Update`、插件、Resource、日志与 Gizmos。
- 章节：{doc}`教程 00：App 运行时 <00-app-runtime/README>`、{doc}`教程 01：插件、资源与系统 <01-plugins-resources-systems/README>`、{doc}`教程 02：调试可视化 <02-debug-visualization/README>`。
- 阶段实践：{doc}`阶段 0：可观测运行时 <stages/stage-00-observability/README>`。

### 阶段 1：ECS、查询与生命周期

- 机制：Entity、Component、Resource、组合查询、`Query` 过滤、`Without`、`ParamSet`、`Commands` 与重开。
- 章节：{doc}`教程 03：ECS 数据模型 <03-ecs-data-model/README>`、{doc}`教程 04：Query 访问冲突 <04-query-access-conflicts/README>`、{doc}`教程 05：Commands 与生命周期 <05-commands-lifecycle/README>`。
- 阶段实践：{doc}`阶段 1：孢子群 <stages/stage-01-spore-swarm/README>`。

### 阶段 2：时间与输入

- 机制：`FixedUpdate`、输入意图、状态切换与暂停边界。
- 章节：{doc}`教程 07：Fixed Update 与时间步 <07-fixed-update/README>`、{doc}`教程 08：输入映射 <08-input-mapping/README>`、{doc}`教程 09：State 与 Run Condition <09-state-run-conditions/README>`。
- 阶段实践：{doc}`阶段 2：俯视角控制器 <stages/stage-02-topdown-controller/README>`。

### 阶段 3：2D 空间与渲染

- 机制：2D 坐标、相机跟随、精灵、图层、Z 顺序与 UI 世界空间。
- 章节：{doc}`教程 10：2D 坐标系 <10-2d-coordinates/README>`、{doc}`教程 11：相机跟随 <11-camera-follow/README>`、{doc}`教程 12：精灵、图层与 Z <12-sprites-layers-z/README>`、{doc}`教程 13：UI 与世界空间 <13-ui-world-space/README>`。
- 阶段实践：{doc}`阶段 3：战术地图 <stages/stage-03-tactical-map/README>`。

### 阶段 4：规则与交互

- 机制：碰撞、伤害、事件驱动规则链与可测试的死亡逻辑。
- 章节：{doc}`教程 14：碰撞最小模型 <14-collision-minimum-model/README>`、{doc}`教程 15：事件驱动伤害 <15-event-driven-damage/README>`、{doc}`教程 16：规则链组成 <16-rule-chain-composition/README>`。
- 阶段实践：{doc}`阶段 4：Arena 生存 <stages/stage-04-arena-survival/README>`。

### 阶段 5：内容与表现

- 机制：资产、动画、音频、UI 反馈与配置。
- 章节：{doc}`教程 17：Asset Server <17-asset-server/README>`、{doc}`教程 18：动画状态 <18-animation-state/README>`、{doc}`教程 19：音频与 UI 反馈 <19-audio-ui-feedback/README>`、{doc}`教程 20：本地化与配置 <20-localization-config/README>`。
- 阶段实践：{doc}`阶段 5：内容表现 <stages/stage-05-content-presentation/README>`。

### 阶段 6：工程化与发布

- 机制：插件化、测试、性能剖析、存档、设置与发布。
- 章节：{doc}`教程 21：插件项目结构 <21-plugin-project-structure/README>`、{doc}`教程 22：测试 ECS 逻辑 <22-testing-ecs-logic/README>`、{doc}`教程 23：性能剖析 <23-performance-profiling/README>`、{doc}`教程 24：存档设置发布 <24-save-settings-release/README>`。
- 阶段实践：{doc}`阶段 6：垂直切片 <stages/stage-06-vertical-slice/README>`。

推荐的阅读顺序是：先读一个章节的 Bevy 说明，再读同页嵌入的 `Rust in Bevy` 内容，运行章节实验，最后观察该机制在本项目中的组合位置。这样可以区分「引擎接口事实」「Rust 语言约束」和「本项目的架构选择」。

## 当前边界

当前根项目已经是可运行的游戏宿主，但仍有明确的工程缺口：资源加载界面、完整存档格式、音量真正作用到播放实例、自动化游戏规则测试、性能基线和目标平台发布配置都应继续沿路线图补齐。

这正是垂直切片的价值：每次加入一个完整、可运行、可回退的机制，而不是先搭建一个规模庞大的空目录，再用「以后实现」掩盖没有验证的数据流。
