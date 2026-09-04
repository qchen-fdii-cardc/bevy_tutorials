# Bevy 教程 17：Asset Server：资源句柄、加载状态与异步边界

很多人把 Bevy 的资源系统理解成“我把某个文件丢进 `AssetServer`，它马上就能用”。这其实忽略了一个关键事实：Bevy 的资源加载是异步的，而 `Handle<T>` 是读写世界时的稳定入口。真正重要的，不是图像“立刻出现”，而是你知道它在哪一帧处于什么状态，并且代码可以在这个状态之间安全地切换。

## 省流版

- `AssetServer` 不是“资源对象”，而是“资源加载与状态查询的入口”。
- `Handle<T>` 是一个安全的引用句柄，允许实体和系统在异步加载期间继续运行。
- 资产的真实生命周期是：`NotLoaded -> Loading -> Loaded`（或 `Failed`）。
- 这章的重点不是“看起来像加载成功”，而是“系统能在世界里正确表达当前状态”。
- 一个稳定的状态资源 + 一个清晰的 `load_state` 断言，通常比盯着屏幕更可靠。

## 运行方式

在本目录执行：

```powershell
cargo test
cargo run
```

示例会在启动时请求加载一个自定义 `.hero` 资源文件。之后，`AssetServer` 会根据注册好的 `HeroAssetLoader` 去解析它，并把状态写进一个 `AssetReport` resource。输出会显示当前状态，如 `Loading` 或 `Loaded`。

## 资产加载在 ECS 中究竟是什么

在 Bevy 里，并不是“某个组件直接持有一张图片”这种直觉写法。更准确的说法是：

- `AssetServer` 负责从磁盘/资源路径拿到资产；
- `Handle<T>` 是对某个资产的稳定引用；
- `Assets<T>` 维护真正的资产数据；
- `System` 读取 `AssetServer` 的状态并决定是否刷新 Sprite、UI 或其他依赖。

这意味着：你不需要等待资源“完全就绪”才能继续构建实体。`Handle<T>` 可以先存在，之后系统再根据 `LoadState` 变更行为。

最小化的状态模型大致是：

```rust
let handle = asset_server.load("hero.hero");
let state = asset_server.load_state(handle.id());
```

这条代码里最重要的不是 `handle` 本身，而是 `state`。因为异步加载正是 ECS 里最容易被误解的部分：资源可能在当前帧中“已经被请求”，但在下一帧才真正到位。

## 为什么这个问题不能用一个 `Resource` 直接装所有内容

很多新手会把一个资源对象写成：

```rust
struct HeroArtifact {
    path: String,
    payload: Option<HeroAsset>,
    is_loaded: bool,
}
```

这看似方便，但很快会踩到几个坑：

1. `HeroAsset` 不是安全地直接存进资源里——它通常来自 `Assets<HeroAsset>`；
2. `is_loaded` 这种布尔值会在异步加载时失真，因为加载和“真正可用”不是一回事；
3. 资源路径、加载状态和实际数据被混在同一层，导致系统边界模糊。

Bevy 更推荐的分层是：

- `Handle<HeroAsset>` 作为引用；
- `AssetServer` 负责状态与路径；
- `Assets<HeroAsset>` 提供真实数据；
- `System` 根据这些状态选择下一步更新。

这套边界不只是风格问题，是真正的安全和可测试设计。

## 一个最小可运行的实验

项目中提供了一个自定义资产文件（见 `assets/hero.hero`）。在启动时，我们请求加载它，并把状态写进一个 `Resource`：

```rust
#[derive(Resource, Default)]
struct AssetReport {
    handle: Option<Handle<HeroAsset>>,
    state: LoadState,
}

fn queue_asset(mut report: ResMut<AssetReport>, asset_server: Res<AssetServer>) {
    if report.handle.is_none() {
        report.handle = Some(asset_server.load("hero.hero"));
    }
}

fn refresh_asset_state(
    asset_server: Res<AssetServer>,
    mut report: ResMut<AssetReport>,
) {
    if let Some(handle) = &report.handle {
        report.state = asset_server.load_state(handle.id());
    }
}
```

注意这里的关键：`report.state` 不描述图片像素本身，而是描述“当前这个资源正在加载、已经加载，还是失败了”。这正是一个状态资源应该承担的职责。

## 加载状态是什么

Bevy 的 `LoadState` 相当直观：

```rust
pub enum LoadState {
    NotLoaded,
    Loading,
    Loaded,
    Failed(Arc<AssetLoadError>),
}
```

这四个状态对应的工程意义是：

- `NotLoaded`：资源还没被请求；
- `Loading`：请求已发出，但还没完成；
- `Loaded`：资源已进入 `Assets<T>`，实际可读；
- `Failed`：路径、格式或 IO 出错，系统需要处理失败分支。

如果你只看一张图片是否“终于出现在屏幕上”，你会忽略中间的状态；如果你把加载状态作为一个真实的 `Resource`，你就能把无数`if loaded`的判断写得更清晰。

## 一个小型测试为何重要

本章的测试不是为了看“截图像不像”，而是为了证明：

1. `AssetServer` 返回了一个有效 handle；
2. 状态在一帧或几帧之后进入 `Loading` 或 `Loaded`；
3. 报告资源中的状态和实际加载状态一致。

这类测试比视觉检查更可靠，因为它断言的是“世界状态”，不是“屏幕看起来对不对”。

## 本章练习

1. 把 `AssetReport` 再拆成 `requested_path` 与 `load_state` 两个独立资源，看看状态边界哪里更清晰。
2. 把“成功后显示 sprite”这一步拆成一个独立 System，观察它应当依赖 `Handle<HeroAsset>` 还是 `LoadState`。
3. 增加一个失败路径，例如加载一个不存在的文件，然后断言状态最终变成 `Failed`。

## 下一章

下一章会把重点从“异步资产如何被观察”转到“动画状态如何作为一条状态机来管理”。在实体和资源之间，状态不只是变量，它也表达了角色的行为边界。

## 延伸阅读

- [Bevy 官方文档](https://bevy.org/learn/)
- [Bevy `0.19` API 文档](https://docs.rs/bevy/0.19.1/bevy/)
- [Bevy 官方示例索引](https://bevy.org/examples/)
- [AssetServer API](https://docs.rs/bevy/0.19.1/bevy/asset/struct.AssetServer.html)

<a id="rust-in-bevy"></a>

```{include} Rust_in_Bevy.md
:heading-offset: 1
```
