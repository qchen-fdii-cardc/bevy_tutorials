# Bevy 教程 05：`Commands`、延迟命令与生命周期：实体何时真正出生和消失

「我刚 `spawn` 的实体怎么查不到」通常不是 Bevy 掉链子；通常是你把命令入队和 World 真正被修改，当成了同一个时刻。

## 省流版

- `Commands` 写入的是一条延迟执行的命令队列，不是立刻改动 World 的同步 API。
- 本章示例在 `Update` 中排队 `spawn` 与 `despawn`，在 `PostUpdate` 观察结果；可见性来自调度边界，不来自你调用方法时的主观感觉。
- `Entity` 是身份句柄，不是对组件的长期引用。生命周期变化后，旧句柄最多还能帮你判断「它是否还活着」。
- 生成、分裂、清理这类副作用规则，优先拆成独立 System，并让日志或测试去验证边界，而不是在一个巨型循环里赌顺序。
- 测试应断言「更新前没有实体、更新后有实体」或「本帧排队销毁、边界后实体已不存在」，这比盯着屏幕猜出生时机更可靠。

## 运行方式

在本目录执行：

```powershell
cargo run
cargo test
```

程序不会创建窗口。它模拟一个最小「孢子分裂」世界：空 World 会先排队生成一颗孢子；生命周期归零后，系统排队生成两个子孢子并销毁父实体；`PostUpdate` 记录每次刷新后的真实群体快照。

## 项目与源码

<a href="https://github.com/qchen-fdii-cardc/bevy_tutorials">GitHub 仓库</a> · <a href="Cargo.toml">查看 <code>Cargo.toml</code></a> · <a href="src/main.rs">查看 <code>src/main.rs</code></a> · <a href="Rust_in_Bevy.md">查看 <code>Rust_in_Bevy.md</code></a>

## 依赖配置

本章仍是纯 ECS 实验：

```toml
bevy = { version = "0.19.1", default-features = false }
```

代码只需要 `App`、Schedule、`Commands`、Entity、Query 与 Resource。窗口、渲染、输入、音频和 UI 都不在本章的因果链上；把它们排除掉，生命周期边界才不会被表现层噪音盖住。

## `Commands` 修改的是未来，而不是现在

[`src/main.rs`](src/main.rs) 里，`seed_initial_spore` 在 World 为空时排队生成第一颗孢子：

```rust
fn seed_initial_spore(
    spores: Query<(), With<Spore>>,
    mut commands: Commands,
    mut log: ResMut<LifecycleLog>,
) {
    if spores.is_empty() {
        commands.spawn((
            DisplayName("seed-g0".to_owned()),
            Spore { generation: 0 },
            Lifetime(2),
        ));
        log.0.push("queued seed-g0".to_owned());
    }
}
```

`commands.spawn(...)` 返回时，这个实体已经进入了命令队列，但它还没有立刻变成当前 Query 可见的 World 状态。本章的第一个测试先断言更新前群体为空，再执行一次 `app.update()`，最后断言 `PostUpdate` 看到一颗真实孢子。你需要记住的结论很朴素：

> `Commands` 的语义是「请求一次 World 变更」，不是「现在立刻获得一个已经稳定存在的实体集合」。

这条边界之所以重要，是因为玩法代码最容易在这里自我欺骗：你以为自己写的是因果链，实际写的是「同一帧里赌调度器怎么刷新命令」。

## 出生、衰减、分裂、清理要拆开

本章把生命周期规则拆成三个 System：

1. `seed_initial_spore`：当群体为空时排队生成初始孢子。
2. `age_spores`：将已有孢子的 `Lifetime` 递减。
3. `split_expired_spores`：对寿命归零的孢子排队生成两个子代，并排队 `despawn` 父实体。

关键代码如下：

```rust
fn split_expired_spores(
    spores: Query<(Entity, &DisplayName, &Spore, &Lifetime)>,
    mut commands: Commands,
    mut log: ResMut<LifecycleLog>,
) {
    for (entity, name, spore, lifetime) in &spores {
        if lifetime.0 == 0 {
            let next_generation = spore.generation + 1;
            commands.spawn((DisplayName(format!("{}-a", name.0)), Spore { generation: next_generation }, Lifetime(1)));
            commands.spawn((DisplayName(format!("{}-b", name.0)), Spore { generation: next_generation }, Lifetime(1)));
            commands.entity(entity).despawn();
            log.0.push(format!("queued split of {}", name.0));
        }
    }
}
```

这里的 `Entity` 只是一个身份句柄，告诉命令系统「之后请把这个实体销毁」。它不是一把可以跨越生命周期变化、长期借住在手里的可变引用。实体存活与否，由刷新后的 World 决定。

## 观察边界，而不是猜边界

本章特意把观测系统放在 `PostUpdate`：

```rust
app.init_resource::<LifecycleLog>()
    .add_systems(Update, (seed_initial_spore, age_spores, split_expired_spores))
    .add_systems(PostUpdate, report_population);
```

在这个实验配置里，`Update` 中排队的生成与销毁，到 `PostUpdate` 时已经体现在群体快照中。于是日志会稳定记录出这样的因果链：

```text
queued seed-g0
post-update population: seed-g0 [g0, ttl=2]
aged seed-g0 to 1
post-update population: seed-g0 [g0, ttl=1]
aged seed-g0 to 0
queued split of seed-g0
post-update population: seed-g0-a [g1, ttl=1], seed-g0-b [g1, ttl=1]
```

这比把 `println!` 塞进同一个大循环里更诚实。你看到的是「刷新之后真实存在的实体集合」，不是一堆尚未落地的副作用想象。

## 自动测试验证的不是 API 记忆，而是生命周期事实

本章两个测试各自验证一条边界：

1. `queued_spawns_become_visible_after_the_update_boundary`：更新前没有孢子，更新后才看到 `seed-g0`。
2. `despawn_and_spawn_are_observed_after_deferred_commands_flush`：寿命为 `0` 的父孢子在边界后消失，同时两个子孢子出现。

这类测试之所以值钱，是因为它们直接断言 World 的输入和输出状态。你以后把日志换成 UI、把实体换成投射物、把分裂换成掉落物，验证方法仍然成立。

## 故障注入

1. 把 `report_population` 错误地放进 `Update`，然后和生成、分裂逻辑混在同一阶段里猜顺序。你会失去稳定的观测边界。
2. 在 `split_expired_spores` 里企图先 `despawn` 再继续把同一个实体当作仍然存在的对象处理。句柄依然是那个句柄，实体生命周期却已经进入待删除队列。
3. 把 `Lifetime` 同时当成「多久后分裂」和「是否应被渲染」的唯一事实。玩法生命周期与表现层状态会重新缠死在一起。
4. 试图在 `spawn` 之后立刻依赖「当前 Query 已经能看见它」的假设组织复杂逻辑。修复方式通常是加清晰的阶段边界、事件或下一帧处理，而不是继续加日志碰运气。

## 本章练习

1. 为孢子添加 `Energy(u8)`，让分裂后的两个子代平分父代能量；写测试验证能量守恒。
2. 将 `LifecycleLog` 改为事件流，再单独写一个记录事件的系统。比较事件与直接写 Resource 日志的边界差异。
3. 添加 `CleanupTag`，让一类实体在另一个阶段统一排队清理。说明这个标记为何比在多个系统里直接 `despawn` 更容易维护。

## 下一章

下一篇处理事件与变更检测：当你不想每帧轮询整个 World 时，应该如何把「发生了什么」压缩成可消费的信号。

## 延伸阅读

- [Bevy `0.19.1`：Commands](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.Commands.html)
- [Bevy `0.19.1`：Entity](https://docs.rs/bevy/0.19.1/bevy/ecs/entity/struct.Entity.html)
- [Bevy `0.19.1`：PostUpdate](https://docs.rs/bevy/0.19.1/bevy/app/struct.PostUpdate.html)
- [Bevy 官方 ECS 示例](https://bevy.org/examples/ecs-entity-component-system/)

<a id="rust-in-bevy"></a>

```{include} Rust_in_Bevy.md
:heading-offset: 1
```
