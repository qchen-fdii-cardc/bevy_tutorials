# Rust in Bevy 16：规则组合与状态编排

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `Attack` / `Armor` / `Health` | `struct` + `Copy` + `derive` | 把战斗事实拆成独立值对象 |
| `Query<(&mut Health, &Attack, &Armor)>` | 组合查询 | 一个 System 接收最小需要的输入，不读整套对象状态 |
| `.chain()` | 系统编排 | 把规则顺序写成显式流程，而不是隐含在函数体里 |
| `saturating_sub` | 标准库方法 | 让伤害不会因数值下溢产生怪异行为 |
| `Option` / `if let` | Rust 模式匹配 | 让“有没有目标、有没有护甲、是否死亡”成为显式条件 |

## 为什么 Rust 适合规则链

Rust 最重要的贡献之一，并不是让代码看起来更高级，而是让你在编译期就确认自己的数据边界。

在本章里，最显著的设计决定是：

- `Attack` 只表示攻击力；
- `Armor` 只表示减伤；
- `Health` 只表示当前生命；
- `resolve_death` 只负责判断是否死亡。

这样的拆分让每个函数都能保持很小的输入输出面。相比之下，`struct Combatant { attack: i32, armor: i32, health: i32 }` 很容易在需求增长时演变成一个大杂烩。Rust 的 `struct` 不会自动帮你建模；真正的设计来自你怎么细化状态、如何限制访问。

## 规则链的 Rust 形态

本章的核心不是一个单独的函数，而是一组作用在同一份 `World` 上的系统：

```rust
fn deal_damage(
    mut attackers: Query<(&Attack, &Target)>,
    mut defenders: Query<(&mut Health, &Armor)>,
) {
    for (attack, target) in &mut attackers {
        if let Ok((mut health, armor)) = defenders.get_mut(target.0) {
            let reduction = (attack.power - armor.reduction).max(0);
            health.0 = health.0.saturating_sub(reduction);
        }
    }
}
```

这里的关键点有两条：

1. 这个函数接收的是必要的输入，不要额外偷拿 `Entity` 别的字段；
2. 它只负责一件事：根据已知参数更新 `Health`。

`max(0)` 和 `saturating_sub` 不是“多写几行代码”，它们直接避免了负数生命值、溢出和不一致的状态。ECS 实验里，极少有单个函数非常复杂的情况下还能保证稳定；小规则和安全边界才是生产力来源。

## `chain()` 不是为了看起来更漂亮

Rust 的系统链写法：

```rust
app.add_systems(
    Update,
    (begin_attack, apply_armor_reduction, apply_health_change, resolve_death).chain(),
);
```

它的语义非常明确：

- 先计算攻击的原始数据；
- 再应用装甲；
- 再写入生命值；
- 最后决定是否死亡和是否清理实体。

如果你把它写成一个大函数：

```rust
fn combat_tick(world: &mut World) {
    // 所有的 if / match / for 都混在这里
}
```

那么问题会在调试时爆炸：到底是哪个判断造成了死亡？是护甲减伤在前还是后？伤害怎么会连带把死亡条件错误触发？

系统链把这些问题变成了显式步骤，这正是 Bevy 的生产价值。

## 从 Rust 设计到系统设计

本章最值得反复练习的，不是某个 API，而是“一条规则只负责一件事”的代码分工：

- `Attack`：攻击的能力；
- `Armor`：抵抗的能力；
- `Health`：当前状态；
- `Dead`：死亡判断。

这些值彼此独立，组合在 Query 内部时才形成一种“当前帧的战斗状态”。这和`enum`、`Option`、`match` 这些 Rust 构造是同一类思维：把复杂状态拆成明确的、可以组合的值。

## 小练习

1. 把 `Armor` 改成 `PhysicalArmor` 和 `MagicArmor`，并让 `deal_damage` 依次处理它们。
2. 把伤害计算抽离成一个纯函数：`fn damage_after_armor(attack: i32, armor: i32) -> i32`，然后在 System 中调用它。
3. 为 `resolve_death` 写一个测试：当 hp `<= 0` 时，`Dead` 组件被写入，且实体在下一帧被 `despawn`。

## 延伸阅读

- [Rust Book：模式匹配与 `if let`](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book：`Option` 和 `Result`](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Bevy `0.19.1`：SystemSet](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.SystemSet.html)
- [Bevy 官方文档](https://bevy.org/learn/)
