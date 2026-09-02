# Rust in Bevy 00：Builder、所有权与唯一 Resource

「链式调用看起来很像语法糖」会遮住关键事实：它是一连串消耗或可变借用 `App` 的普通 Rust 方法调用。

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `App::new().add_systems(...).run()` | Builder 风格 API 与方法链 | 用类型约束运行时装配顺序 |
| `#[derive(Resource)]` | derive macro 与 trait 实现 | 将类型声明为 World 中唯一共享状态 |
| `Res<Time>` | 共享借用 `&T` 的系统参数包装 | 系统可读、不可改 Time |
| `ResMut<RuntimeStats>` | 独占借用 `&mut T` 的系统参数包装 | 系统获得唯一写入权 |
| `impl Default` | `Default` trait | 将初始状态从装配代码中抽离 |

## Builder 链不是魔法

```rust
App::new()
    .insert_resource(RuntimeStats::default())
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .run();
```

每一步都返回可继续配置的 `App`，最后 `run(self)` 取得应用所有权并进入事件循环。Rust 通过 `self`、`&mut self` 和返回值控制调用权限：配置完成后，运行时不再把可修改的 `App` 留在当前作用域。

这是一种常见的 Rust API 设计：构建阶段允许累积配置，终结方法消耗构建器，阻止「程序运行后继续偷偷改初始化图」的半成品状态。

## `Default` 表达可用的零配置

`RuntimeStats::default()` 集中给出初始时间、帧数和日志节流点。它比散落的 `0.0`、`0` 更可审计：读者能在一个 `impl Default` 中看到所有初始不变量。

当类型存在真正安全的默认值时，实现 `Default`。当每个字段都必须由调用方决定，例如存档路径或网络地址，强行提供默认值只会制造伪配置。

## 借用检查器如何成为调度信息

```rust
fn advance_runtime(time: Res<Time>, mut stats: ResMut<RuntimeStats>)
```

这段签名相当于声明「读取 `Time`，独占修改 `RuntimeStats`」。普通 Rust 中，同一时刻不能同时拥有同一值的共享借用和可变借用；Bevy 将相同规则提升到 System 调度层，提前发现世界状态的竞争。

不要用 `Mutex` 把所有状态包起来逃避这一层。游戏逻辑的同步边界应先由 ECS 读写集合表达；锁会把错误推迟到运行时，并让并行调度失去可见性。

## 小练习

为 `RuntimeStats` 添加 `is_reporting_due(&self) -> bool`，只让纯判断逻辑进入方法。System 仍负责读取 Time 和修改 Resource。这个分工体现 Rust 中常用的模式：数据类型维护局部不变量，外部协调者负责 I/O 与生命周期。

## 延伸阅读

- [Rust Book：方法语法](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust Book：所有权](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book：Trait](https://doc.rust-lang.org/book/ch10-02-traits.html)