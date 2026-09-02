# Rust in Bevy 03：组合数据、Newtype 与可测试规则

「ECS 放弃对象」不等于放弃建模。它要求你用小类型和组合来表达模型，而不是把变化原因绑死在一个巨型 struct 中。

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `struct Position(Vec2)` | Newtype pattern | 为坐标赋予位置语义，避免裸 `Vec2` 混用 |
| `struct Player;` | 零大小 marker type | 用类型表达阵营或角色分类 |
| `Query<(&mut Position, &Velocity)>` | 元组、可变借用与解构 | 只给规则需要的最小数据集合 |
| `#[derive(Component)]` | derive macro | 将普通 Rust 数据纳入 World 存储 |
| `#[cfg(test)]` | 条件编译 | 测试代码不进入发布构建 |

## Newtype 防止语义漂移

`Position(Vec2)` 与 `Velocity(Vec2)` 的底层数据都是 `Vec2`，语义却完全不同。若函数直接接受两个 `Vec2`，参数位置写反仍可能编译；newtype 让这种错误在类型检查阶段暴露。

```rust
struct Position(Vec2);
struct Velocity(Vec2);
```

这是一项廉价的领域建模技术。为真实不同的概念建立不同类型，别让「恰好都是两个 f32」替代语义。

## 元组查询是局部组合

`Query<(&mut Position, &Velocity)>` 并没有创建临时「可移动对象」。它对每个匹配实体借出两个字段，并通过元组解构交给 System。规则只看见位置和速度，因此不会意外依赖名称、阵营或生命值。

这种参数级组合比在 `Player` 上堆方法更稳健：投射物加入同样两个 Component 后，自动复用移动规则。

## 测试依赖状态，而非画面

```rust
assert_eq!(app.world().get::<Position>(moving_entity).unwrap().0, Vec2::new(6.0, 2.0));
```

测试直接构建输入 World，执行一次更新，断言输出 World。它没有启动窗口，也不关心渲染帧率。这正是 Rust 测试的优势：将规则写成可注入依赖的纯数据变换，测试速度和错误定位都会改善。

`#[cfg(test)]` 让测试模块只在 `cargo test` 构建，避免示例二进制携带仅用于断言的代码。

## 小练习

为 `Health(u32)` 添加 `fn is_dead(&self) -> bool`。不要在它内部 despawn 实体：该方法只负责局部事实判断，生命周期副作用仍由 System 与 `Commands` 协调。

## 延伸阅读

- [Rust API Guidelines：Newtype](https://rust-lang.github.io/api-guidelines/type-safety.html)
- [Rust Book：测试](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust Reference：条件编译](https://doc.rust-lang.org/reference/conditional-compilation.html)