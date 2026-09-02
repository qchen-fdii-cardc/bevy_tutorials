# Bevy 教程 03：实体没有行为：ECS 中的身份、事实与规则

「给 `Player` 塞一个万能 struct」不会让对象更完整；它只会让每项新规则都去改同一个状态垃圾场。

## 省流版

- `Entity` 只是稳定身份，不携带 Rust 方法和继承层级；`Component` 才是关于该身份的事实。
- `Position` 与 `Velocity` 可以组合在玩家、敌人、投射物或任何新实体上。移动规则只查询这两个事实，不关心标签。
- `Player`、`Enemy` 是零大小的标记 Component。它们表达分类，不能替代位置、速度、生命值等独立事实。
- `System` 是对匹配数据集合的变换规则。本章的 `move_entities` 只会移动同时拥有 `Position` 与 `Velocity` 的实体。
- Unit test 直接构造 World、运行一次 `Update`，并证明静止实体没有被移动。可测试的数据边界，比「对象看起来很面向对象」有用得多。

## 运行方式

在本目录执行：

```powershell
cargo run
cargo test
```

程序不创建窗口。它在内存中生成玩家和敌人，推进一次模拟，然后打印各自位置。这个刻意贫瘠的实验删除了渲染、输入和时间，留下 ECS 的因果链。

## 依赖配置

本章是纯 ECS 实验，依赖配置因此更小：

```toml
bevy = { version = "0.19.1", default-features = false }
```

`App`、World、Entity、Component、Query、Resource 与 `Vec2` 均可在此配置下使用。窗口、渲染、输入、音频、UI、资产加载和 Gizmos 都不属于本章因果链，因此全部排除。只测试数据变换时，先把图形运行时移出依赖图，错误定位和编译速度都会变得更诚实。

## 一个身份，多个事实

[`src/main.rs`](src/main.rs) 的玩家由四个 Component 组成：

```rust
commands.spawn((
    Name("player"),
    Player,
    Position(Vec2::ZERO),
    Velocity(Vec2::new(3.0, 0.0)),
));
```

这里并没有 `Player::update()`。同一个实体拥有一个 `Entity` ID，World 用该 ID 把 `Name`、`Player`、`Position` 和 `Velocity` 关联起来。敌人换了 `Enemy` 标签和初始数据，却复用完全相同的位置与速度事实。

> ECS 的第一个约束是：**先问某条数据是否独立变化，再决定它是否应成为 Component。**
>
> 「所有玩家都有」不是理由；「移动规则需要独立读取和写入它」才是理由。

## System 不服务对象，它服务查询

```rust
fn move_entities(
    mut movers: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in &mut movers {
        position.0 += velocity.0;
    }
}
```

这个 System 选择的是拥有 `Position` 和 `Velocity` 的实体集合。它不会移动只有 `Position` 的静态装饰物，也不需要为玩家和敌人各复制一份移动逻辑。新类型只要组合相同事实就自动进入规则范围。

`SimulationStep` 是 Resource，因为整个模拟共享唯一的步数。它和实体位置的归属不同：位置属于一个实体，步数属于整个 World。把两者都塞进 `Player`，只是把状态所有权说反了。

## 自动测试就是最小实验

测试创建一个可移动实体和一个只有 `Position` 的静止实体，执行 `app.update()`，验证前者位置变为 `(6, 2)`，后者仍为 `(7, 8)`。这个断言验证的是查询边界正确，不是 `Vec2` 加法本身。

当玩法规则复杂起来，优先测试 System 的输入状态和输出状态。窗口截图最多证明某一帧看起来像对的；World 断言可以证明哪个事实被谁改变。

## 故障注入

1. 从玩家移除 `Velocity`。再次运行后，玩家不再移动，敌人仍移动。规则由 Component 组合决定。
2. 将 `Enemy` 加到玩家实体。移动结果不变，因为 `move_entities` 根本不读取这个标签。标签不是行为。
3. 在查询中错误加入 `With<Player>`。敌人停止移动，代码仍能编译；你把可复用规则收窄成玩家特权，这就是数据模型的回归。
4. 把 `SimulationStep` 改为 Component 并加到玩家。多玩家出现后你必须决定读哪一个步数。它原本是共享状态，Resource 才是正确归属。

## 本章练习

1. 添加 `Health(u32)`，让玩家和敌人都具有生命值，但不要让移动系统读取它。
2. 添加 `Projectile` 标记和一个投射物实体，验证它无需新移动 System 就会移动。
3. 写一个测试，证明只有拥有 `Health` 的实体会被「受伤」规则修改。

## 下一章

下一篇处理 `Query` 的读取、写入与冲突：当两个 System 都想修改相同 Component，Bevy 和 Rust 正在阻止哪一种真实的数据竞争。

## 延伸阅读

- [Bevy `0.19.1`：Entity](https://docs.rs/bevy/0.19.1/bevy/ecs/entity/struct.Entity.html)
- [Bevy `0.19.1`：Component](https://docs.rs/bevy/0.19.1/bevy/ecs/component/trait.Component.html)
- [Bevy `0.19.1`：Query](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.Query.html)
- [Bevy 官方 ECS 示例](https://bevy.org/examples/ecs-entity-component-system/)

## Rust in Bevy

```{include} Rust_in_Bevy.md
:heading-offset: 1
```