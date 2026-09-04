# Bevy 教程 16：从一次攻击到一条规则链：组合系统而非巨型 `match`

「把所有战斗逻辑塞进一个 `update` 函数」听起来像在写游戏代码，实际上是在把规则压进一张长长的 `match` 表。ECS 的价值就在于：每条规则负责自己的输入和输出，而不是一口气吞掉整个战斗状态机。

## 省流版

- 本章的核心不是「能打到怪」而是「为什么伤害被拆成多条规则更稳」。
- `Attack`、`Armor`、`Health` 分别是不同事实，应该分别建模，而不是把它们压进一个万能结构体。
- 一次攻击的真实流程是：先计算原始伤害，再应用防御，再更新生命值，再决定是否死亡。
- 规则链更容易测试，因为每一步都能写成一个很小的 System，并在世界状态上断言。
- 一旦你开始写大 `match`，往往是把状态机和系统边界都搞混了；这章的目标就是把它拆回来。

## 运行方式

在本目录执行：

```powershell
cargo run
cargo test
```

程序会创建一个玩家和一个敌人：玩家持有 `Attack` 与 `Armor`，敌人持有 `Health`。一次更新后，系统会依次应用攻击、扣减护甲、写回生命值，并输出最终 hp，以及是否在本帧死亡。

## 一个攻击到底包含哪些事实

在真实游戏中，攻击并不只是一个数字。它至少会涉及：

- 谁发起攻击；
- 攻击的基础威力；
- 目标的防御能力；
- 目标当前生命值；
- 目标是否已经死亡。

如果你把这些压成一个 `Combatant` 结构体，那么每次新增规则都要改同一个对象，规则之间也越来越互相依赖。

本章用三类组件表示最小事实：

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Attack {
    power: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Armor {
    reduction: i32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
struct Health(i32);
```

这意味着：

- `Attack` 只描述「造成多少伤害」；
- `Armor` 只描述「被减多少伤害」；
- `Health` 只描述「当前生命值」；
- 生和死由 `resolve_death` 这类系统决定，而不是由某个字段埋在巨型结构体里。

## 为什么这章不是写 `match`

很多同学会在一开始写：

```rust
match target {
    Enemy => ...
    Player => ...
    Projectile => ...
}
```

但这种写法会立刻出现几类问题：

1. 一个规则里同时处理攻击、护甲、死亡和粒子效果，职责混在一起。
2. 每增加一种状态，都要回到大 `match` 里改一条分支。
3. 测试会很脆弱：你得把一整串行为串起来跑，不能看出是哪一步出错。

ECS 则是让每个 System 只服务一类输入：

```rust
fn deal_damage(mut attackers: Query<(&Attack, &Target)>, mut defenders: Query<(&mut Health, &Armor)>) {
    // 先读取所有攻击事实，再写回生命值
}
```

这条链的每一步都带有边界：

- 先算基础伤害；
- 再应用护甲；
- 再写回 `Health`；
- 最后检查是否死亡。

说白了，ECS 不是在模拟 OOP 的对象方法；它是在把一个规则链写成一个数据流。

## 最小示例：伤害链

[`src/main.rs`](src/main.rs) 的例子非常小，但它刻意把规则链分成三步：

```rust
fn resolve_attack_damage(
    mut actors: Query<(&Attack, &Target)>,
    mut defenders: Query<(&mut Health, &Armor)>,
) {
    for (attack, target) in &actors {
        if let Ok((mut health, armor)) = defenders.get_mut(target.0) {
            let damage = (attack.power - armor.reduction).max(0);
            health.0 = health.0.saturating_sub(damage);
        }
    }
}
```

对应的因果链是：

1. `Attack` 提供基础伤害；
2. `Armor` 提供减伤；
3. `Health` 被更新；
4. `resolve_death` 再决定倒地、消失或触发其他事件。

这里的关键点不是数学能不能算出来，而是「处理顺序」必须稳定。你先算伤害还是先算护甲，决定了规则语义。把这些在系统里拆开后，不仅合理，而且可测试。

## 为什么顺序也属于设计

本章的代码会把规则拆成顺序链：

```rust
app.add_systems(
    Update,
    (
        begin_attack,
        apply_armor_reduction,
        apply_health_change,
        resolve_death,
        report_battle_state,
    )
        .chain(),
);
```

`chain()` 的意义不是“看起来更高级”，而是让系统顺序可见：

- `begin_attack` 先准备攻击信息；
- `apply_armor_reduction` 再决定伤害减免；
- `apply_health_change` 更新最终 hp；
- `resolve_death` 再决定清理和碎片效果。

一旦顺序被隐含在大函数中，就很难知道到底哪一步更新了什么。如果顺序被写成显式系统链，就很容易调用日志、断言和调试工具进行定位。

## 真实世界中的故障：顺序和状态混在一个函数里

故障案例总是非常典型：

1. 先扣血，再应用护甲；
2. 目标死亡后仍然继续被攻击；
3. 护甲与攻击来自不同实体，却被写进同一个 `Damage` struct；
4. 让 `resolve_death` 在 `apply_health_change` 之前运行，导致死亡条件判断过早失真。

这时编译器不会直接报错，因为 Rust 只保证类型和借用安全，不保证你设计的规则顺序是正确的。真正需要的，是把顺序写成系统链，再用断言说明世界状态。

## 自动测试：证明规则链成立

本章会有两个关键测试：

1. 证明攻击不足以穿透护甲：小于护甲值时最终 hp 不变；
2. 证明攻击足够强时，hp 会被减到 0，并触发死亡判定。

这些测试的价值在于：

- 它们不是 UI 图像测试；
- 它们不是只看“输出看起来像对”；
- 它们直接断言 `Health` 和 `Dead` 状态在 `World` 中是怎样变的。

你只要保持系统链稳定，后面再加入重击、暴击、状态异常、减伤buff，逻辑都能沿着同样的边界继续扩展，而不需要重写整个 `match`。

## 本章练习

1. 把 `Armor` 拆成 `PhysicalArmor` 和 `MagicArmor`；写一个测试说明两种护甲在同一条规则链上的不同影响。
2. 从 `Attack` 结构体中去掉直接写入 World 的逻辑，保留纯函数 `fn attack_damage(attack: &Attack, armor: &Armor) -> i32`，然后让一个 System 调用它。
3. 写一个 `Dead` 标记组件，用 `resolve_death` 在生命值降到 0 时 `despawn` 目标，并验证系统清理仍然遵守规则边界。

## 下一章

下一章会把注意力转向 `Commands` 与生命周期：实体出现、销毁和复用的时机不只是“代码调用了某个函数”，而是由调度边界和命令队列决定的。系统链解决的是规则顺序，`Commands` 解决的是当下世界是否已经发生更新。

## 延伸阅读

- [Bevy 官方 ECS 设计文档](https://bevy.org/learn/)
- [Bevy `0.19.1`：Schedule](https://docs.rs/bevy/0.19.1/bevy/ecs/schedule/struct.Schedule.html)
- [Bevy `0.19.1`：SystemSet](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.SystemSet.html)
- [Bevy 官方示例索引](https://bevy.org/examples/)

<a id="rust-in-bevy"></a>

```{include} Rust_in_Bevy.md
:heading-offset: 1
```
