# Rust in Bevy 04：借用与 Query 访问声明

Query 参数不是「从世界里随便拿数据」的语法糖。它是系统在调用前写出的读写合同。

## 本章唯一主题

将 Rust 的共享借用 `&T`、可变借用 `&mut T` 映射到 Query 的访问集合，并用筛选器或 `ParamSet` 解决真实重叠。

```rust
fn move_players(
    mut players: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in &mut players {
        transform.translation += velocity.0.extend(0.0);
    }
}
```

- 同一系统不能同时取得可能指向同一组件的 `&T` 与 `&mut T`。
- `With<Player>`、`Without<Player>` 要表达实体集合确实互斥的事实，不能用来掩盖重叠。
- 读阶段和写阶段确需分离时，`ParamSet` 让它们按顺序取得访问权。

## 设计用法

冲突优先级：缩小 Query → 用组件标签证明集合互斥 → 拆分系统或数据 → 最后才使用 `ParamSet`。编译器拒绝访问的原因通常是数据模型没有写清。

`Query::get` 返回 `Result`，因为目标实体可能不存在或不匹配；不要用 `unwrap` 将普通游戏状态变成崩溃。

## 练习

写出两个都修改 `Health` 的 Query，先观察冲突，再用 `With<Player>` 与 `With<Enemy>` 表达互斥集合。
