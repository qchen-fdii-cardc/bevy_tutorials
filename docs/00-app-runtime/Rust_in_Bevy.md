# Rust in Bevy 00：可变绑定与 Builder 调用链

「链式 API 很优雅」并不等于它没有状态变化。`App` 的配置阶段就是 Rust 中可变借用最直观的练习场。

## 本章唯一主题

理解 `let mut`、`&mut self` 与方法链：配置函数临时借用同一个 `App`，每次调用结束后归还借用，最后才以 `run(self)` 消耗它。

```rust
let mut app = App::new();
app.add_plugins(DefaultPlugins)
   .add_systems(Startup, setup);
app.run();
```

- `app` 必须可变，因为 `add_plugins` 和 `add_systems` 修改其内部配置。
- 方法返回 `&mut App`，因此可以继续调用；这不是复制出多个 `App`。
- `run(self)` 取得所有权。调用后不能再访问 `app`，这正好表达「配置结束，运行时开始」的边界。

## 设计用法

把构建期配置集中在 `main` 或一个单独的 `build_app` 函数中；运行中的世界状态交给 `Resource` 和 `Component`。不要在系统中重新配置 `App`，也不要为了保留一个配置对象而无意义地 `clone`。

检查方法签名比猜测更可靠：`&self` 只读、`&mut self` 修改但不接管、`self` 接管值。这个读取签名的习惯会贯穿后面的 Query、Commands 与资产句柄。

## 练习

将初始化提取为 `fn build_app() -> App`，在返回前加入一个 `Startup` 系统。说明为什么 `run` 应留在调用方。
