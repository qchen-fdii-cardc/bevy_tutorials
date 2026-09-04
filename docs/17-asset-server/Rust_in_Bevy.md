# Rust in Bevy 17：Handle、AssetServer 与真正的异步边界

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `AssetServer` | `Res<AssetServer>` | 资源加载入口与状态查询器 |
| `Handle<T>` | `Handle<HeroAsset>` | 资源引用句柄，允许异步加载继续进行 |
| `Resource` | `struct` + `Default` | 储存当前状态，不直接存大块数据 |
| `System` | `fn` + `ResMut<T>` | 读取状态并写回一个更清晰的事实 |

## 核心思想

Rust 对“可变性”和“所有权”非常严格，而 Bevy 的资源管理则把这个原则应用到异步资产上。你不能用一个单独的 `HeroAsset` 字段假设它“已经准备好”，因为它的加载全生命周期是拆成多个阶段的。

`AssetServer` 和 `Handle<T>` 的价值在于：

- 它把文件路径、加载状态和实际数据拆离；
- 资源的数据并不在 `Resource` 里，而是存在 `Assets<T>` 中；
- 系统真正关心的是“当前是什么状态”，而不是“我是否碰巧觉得它已可用”。

这和 Rust 的借用模型非常契合：一个对象可以在不持有真实数据时，先被引用，然后在适当时刻再拿到实际值。

## 为什么 `Handle<T>` 比直接存 `Asset` 更合理

如果你写：

```rust
struct HeroToken {
    asset: HeroAsset,
}
```

你很快会遇到一个问题：`HeroAsset` 是资产数据，不是一个随时可用的常量。它本来就需要读文件、解析、缓存和引用管理。`Handle<HeroAsset>` 恰恰表达了“我有这个资源的引用”，而不是“我已经拥有这个资源的全部数据”。

这能避免两种典型错误：

1. 在加载前读取图片内容，导致空值或未初始化；
2. 把加载状态和数据内容塞进同一个结构体，系统边界就消失了。

## 本章中最重要的 Rust 结构

```rust
#[derive(Resource, Default)]
struct AssetReport {
    handle: Option<Handle<HeroAsset>>,
    state: LoadState,
}
```

这里的几个设计点很关键：

- `Option<Handle<HeroAsset>>` 表示“可能还没有请求完成”；
- `LoadState` 是状态机；
- `System` 可以借助这些事物判断是否安全地继续绘制或切换 UI。

这是 Bevy 里一个极简而真实的示例：状态本身就成了一等的数据。

## 小练习

1. 把 `AssetReport` 拆成一个只记录 `handle` 的资源和一个只记录 `state` 的资源，观察边界是否更清晰。
2. 把 `LoadState::Failed` 分支写成一个独立系统，看看错误处理是否更容易测试。
3. 在 Rust 侧写一条 `assert!(matches!(state, LoadState::Loading | LoadState::Loaded));` 式断言，验证你的状态模型确实在世界中生效。

## 延伸阅读

- [Rust Book：枚举](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [Rust Book：测试](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Bevy 官方文档](https://bevy.org/learn/)
- [Bevy AssetServer API](https://docs.rs/bevy/0.19.1/bevy/asset/struct.AssetServer.html)
