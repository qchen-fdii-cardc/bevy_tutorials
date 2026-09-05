# Rust in Bevy 03：Struct、新类型与领域事实

「玩家」不是一个必须装下所有字段的大结构体。Rust 的小类型让 ECS 的事实边界可见，也让错误更早暴露。

## 本章唯一主题

用结构体和元组新类型为不同领域量命名。

```rust
#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Health(u16);
```

- `Position(Vec2)` 与裸 `Vec2` 的内存成本相近，却能阻止把速度误当位置传递。
- 字段私有时，模块可保证不变量；需要读写时再提供方法。
- Component 应描述稳定事实。一次碰撞的结果、全局难度或临时局部变量各有更合适的承载位置。

## 设计用法

先为游戏中的名词建类型，再决定实体拥有哪些组件。新类型不是形式主义：它把「这个数值代表什么」交给编译器检查。

本章只解决数据命名与归属；下一章才讨论多个系统同时访问这些类型的规则。

## 练习

将一个 `f32` 生命值改为 `Health(u16)`，只通过 `Health::take` 修改它，并在方法内保证结果不小于零。
