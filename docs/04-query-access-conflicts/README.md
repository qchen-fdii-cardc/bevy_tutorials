# Bevy 教程 04：`Query` 的读取、写入与冲突：借用检查器在保护什么

「编译器嫌两个查询冲突」通常不是它不懂你的意图；通常是你的数据模型还没把读写边界讲清。

## 省流版

- `Query<&Health>` 声明只读访问，`Query<&mut Health>` 声明独占写入。System 参数就是 ECS 的并发契约。
- 两个独立 System 可以分别修改玩家与敌人的 `Health`，但要用 `With<Player>` 和 `With<Enemy>` 证明实体集合互斥。
- 在同一 System 中既读敌人又写玩家的同类 Component，使用 `ParamSet` 分开读取阶段和写入阶段。
- `Without<T>` 只有在它准确表达实体集合互斥时才是修复；它不是让编译器闭嘴的胶带。
- 本章的测试验证两种安全模式：互斥过滤器允许并列写入，`ParamSet` 允许先读后写。

## 运行方式

在本目录执行：

```powershell
cargo run
cargo test
```

程序创建一个生命值为 `8` 的玩家与生命值为 `5` 的敌人。一次更新后，玩家被治疗到 `10`，敌人受伤到 `2`，并输出两条状态日志。

## 项目与源码

<a href="https://github.com/qchen-fdii-cardc/bevy_tutorials">GitHub 仓库</a> · <a href="Cargo.toml">查看 <code>Cargo.toml</code></a> · <a href="src/main.rs">查看 <code>src/main.rs</code></a> · <a href="Rust_in_Bevy.md">查看 <code>Rust_in_Bevy.md</code></a>

## 依赖配置

本章和第 03 章一样是纯 ECS 实验：

```toml
bevy = { version = "0.19.1", default-features = false }
```

代码只需要 App、World、Component、Query、Resource 和 `ParamSet`。窗口、渲染、输入、音频与 UI 全部不在本章的因果链上。

## 一个 Component 类型不等于一份数据

玩家和敌人都拥有 `Health`，但它们是不同实体上的不同 Component 实例。下面两个 System 都写 `Health`：

```rust
fn heal_players(mut players: Query<&mut Health, With<Player>>) { /* ... */ }

fn damage_enemies(mut enemies: Query<&mut Health, With<Enemy>>) { /* ... */ }
```

Bevy 能依据 `With<Player>` 和 `With<Enemy>` 看见这两个集合没有交集，因此它允许两个 System 注册在同一调度中。这个事实有两个前提：实体分类真的互斥，并且没有实体同时带着 `Player` 与 `Enemy`。

> `Query` 的过滤器是数据访问声明的一部分。**只有类型正确、集合也正确，借用安全才成立。**

给敌人错误地加上 `Player` 会让这个前提消失。此时修复方式取决于游戏规则：将阵营建模为单一 `Faction` Component、修改过滤条件，或拆开状态。继续堆 `Without` 只是把架构债务藏进泛型参数。

## 读写同类数据时，分开阶段

`transfer_health` 需要统计敌人数量，再将数量加到玩家生命值。它既读取又修改 `Health`：

```rust
fn transfer_health(
    mut queries: ParamSet<(
        Query<&Health, With<Enemy>>,
        Query<&mut Health, With<Player>>,
    )>,
) {
    let enemy_count = queries.p0().iter().count() as i32;

    for mut health in &mut queries.p1() {
        health.0 += enemy_count;
    }
}
```

`ParamSet` 保证 `p0()` 的只读借用在进入 `p1()` 的可变借用前结束。它描述的是「先收集事实，再变更事实」的阶段边界。需要同时修改两个实体时，优先考虑事件、暂存值或重新设计 Component，别试图在同一循环中绕过别名规则。

## 故障注入：错误各不相同

1. 从 `damage_enemies` 删除 `With<Enemy>`。两个 System 都可能写任意 `Health`，Bevy 会拒绝并发访问声明。恢复精确过滤器。
2. 在 `transfer_health` 中直接声明两个 `Query` 参数，分别读取敌人与修改玩家。即使你认为标签互斥，复杂查询也容易掩盖别名风险。将它们放进 `ParamSet`，让阶段边界可见。
3. 给一个实体同时加上 `Player` 和 `Enemy`，运行第一项测试。断言会暴露生命值与预期不一致；这个失败说明标记 Component 没有维护互斥语义。
4. 将 `report_health` 放到 `Startup`。日志只记录初始化状态，随后治疗和伤害仍在每帧发生。观测系统的调度位置错误，会制造「规则没运行」的假象。

## 本章练习

1. 以 `Faction` enum Component 替换 `Player` 与 `Enemy` 标记，并让两个规则只查询对应阵营。
2. 新增 `Armor(i32)`，把伤害公式变为 `max(0, damage - armor)`；写测试验证护甲不会使生命值上升。
3. 把治疗与伤害改为事件生产者和事件消费者。比较事件队列与直接写 Query 的数据流和调度代价。

## 下一章

下一篇处理 `Commands` 与延迟命令：实体的出生和消失何时真正写进 World，以及为什么「我刚 spawn 的实体怎么查不到」不是引擎故障。

## 延伸阅读

- [Bevy `0.19.1`：Query](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.Query.html)
- [Bevy `0.19.1`：ParamSet](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.ParamSet.html)
- [Bevy `0.19.1`：访问冲突](https://docs.rs/bevy/0.19.1/bevy/ecs/query/struct.QueryFilter.html)
- [Bevy 官方 ECS 示例](https://bevy.org/examples/ecs-entity-component-system/)

<a id="rust-in-bevy"></a>

```{include} Rust_in_Bevy.md
:heading-offset: 1
```