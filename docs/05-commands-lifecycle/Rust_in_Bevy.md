# Rust in Bevy 05：所有权转移、身份句柄与延迟副作用

「World 里有一份数据」的前提，是你已经把那份数据的所有权交了进去。ECS 没有替你取消 Rust 的所有权规则；它只是把规则搬到了系统边界上。

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `commands.spawn((DisplayName(...), ...))` | move semantics | 组件值被移动进 World，之后由 ECS 持有 |
| `Entity` | 轻量 Copy 句柄 | 用稳定身份索引实体，不把组件借用跨边界长期保存 |
| `DisplayName(String)` | 拥有所有权的堆分配字符串 | 实体名称在调用栈结束后仍能存活 |
| `Vec<String>` 日志 Resource | 可增长缓冲区 | 将观测结果累积起来，供打印或测试断言 |
| `#[derive(Default)]` Resource | 默认构造 | 为可重复测试提供明确初始状态 |

## `spawn` 背后是所有权转移

```rust
commands.spawn((
    DisplayName("seed-g0".to_owned()),
    Spore { generation: 0 },
    Lifetime(2),
));
```

这里的 `String`、`Spore` 与 `Lifetime` 都被 move 进了命令队列，最终进入 World。调用点之后，你不再拥有这些值；若还想在外面继续复用同一份数据，就必须克隆、重建，或把真正共享的信息提炼成 Resource。

这和 Rust 的普通函数调用没有本质区别。ECS 没有神秘地「帮你保留一份副本」；数据归属仍然必须清楚。

## `Entity` 是句柄，不是引用

`Entity` 很适合表达「之后我要操作谁」：

```rust
commands.entity(entity).despawn();
```

它的意义接近一个稳定 ID，而不是 `&mut T`。你不能把它想成「我手里握着这个实体全部组件的活引用」。实体可能在边界后被销毁、复用或失效；真正的数据访问仍要回到 Query、World 或命令系统。

## `String` 比 `&str` 更适合动态派生出的实体名

本章子孢子的名字是运行时拼出来的：

```rust
DisplayName(format!("{}-a", name.0))
```

`format!` 产生的是 `String`，它拥有自己的内存，可以安全地随实体一起存活。若组件改为 `&str`，你就必须额外保证底层字符串在整个实体生命周期内有效；对动态名称来说，这通常徒增约束。

## 日志 Resource 是一种受控副作用缓冲区

`LifecycleLog(Vec<String>)` 把系统的观测输出集中到一个地方。它不替代事件，也不替代测试；它的价值在于让副作用从 `println!` 这种即时 IO 退回成可检查的数据。

这正是 Rust 常见的设计手法：先把结果收集进普通数据结构，再决定如何显示、断言或持久化。

## 小练习

把 `split_expired_spores` 中生成子代名称的逻辑提取为：

```rust
fn child_name(parent: &DisplayName, suffix: char) -> DisplayName
```

让这个函数只负责构造拥有所有权的新名称，不接触 ECS。这样你可以单独测试命名规则，而生命周期副作用仍留在 System 内。

## 延伸阅读

- [Rust Book：所有权](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Book：堆分配字符串](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust API Guidelines：C-CUSTOM-TYPE](https://rust-lang.github.io/api-guidelines/type-safety.html)
