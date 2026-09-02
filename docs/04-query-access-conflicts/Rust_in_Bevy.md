# Rust in Bevy 04：可变别名、类型别名与阶段化访问

「借用检查器妨碍写代码」是把数据竞争当作自由。它要求你标出谁在何时拥有写权限，正好对应游戏规则的真实边界。

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `Query<&Health>` | 共享借用 | 多个规则可以同时观察生命值 |
| `Query<&mut Health, With<Player>>` | 独占借用加类型过滤 | 写权限仅限玩家集合 |
| `ParamSet<(..., ...)>` | 生命周期受限的分阶段访问 | 同类数据先读后写，避免别名 |
| `type EnemyHealthReader = ...` | 类型别名 | 为复杂泛型签名赋予业务含义 |
| `#[cfg(test)]` | 条件编译 | 只为测试保留辅助规则 |

## `&mut` 的真正含义

Rust 保证同一时刻一份数据要么有任意数量的 `&T`，要么只有一个 `&mut T`。这条规则防止未定义行为和逻辑竞争，它不是可选的风格建议。

Bevy 从 System 参数推导这份访问集合。`Query<&mut Health, With<Player>>` 明确要求玩家的生命值写权限；`Query<&mut Health, With<Enemy>>` 明确要求敌人的写权限。集合互斥时，两个 System 可以安全并发。

## Filter 是证明的一部分

`With<Player>` 不只是方便筛选。它参与「两个 Query 是否可能命中同一 Component」的证明。若实体能同时拥有 `Player` 与 `Enemy`，两个写者的互斥前提已经失效。

因此，Component 设计需要维护不变量。阵营天然互斥时，`Faction` enum 往往比多个独立 marker 更容易保证正确状态；类型系统不能替你自动补上所有领域规则。

## `ParamSet` 与分阶段算法

```rust
let enemy_count = queries.p0().iter().count() as i32;
for mut health in &mut queries.p1() {
    health.0 += enemy_count;
}
```

`ParamSet` 让 `p0()` 与 `p1()` 的访问不能同时存活。代码先产生一个普通数值 `enemy_count`，只读借用结束，然后进入写阶段。这是 Rust 常见的两阶段模式：先收集所需事实，再实施变更。

类型别名 `EnemyHealthReader` 与 `PlayerHealthWriter` 去掉重复的泛型噪音，并把两个阶段的角色写进名称。类型别名不创建新类型，它只是让复杂签名可读。

## 小练习

写一个 `Damage(i32)` newtype 和 `fn apply_damage(health: &mut Health, damage: Damage)`。让函数只处理数值变化；目标筛选、事件读取和日志仍留给 System。这样的函数可脱离 ECS 测试，也不会无意取得 World 的过大访问权限。

## 延伸阅读

- [Rust Book：引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Reference：类型别名](https://doc.rust-lang.org/reference/items/type-aliases.html)
- [Bevy `ParamSet` 文档](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.ParamSet.html)