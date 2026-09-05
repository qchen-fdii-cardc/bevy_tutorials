# Rust in Bevy 12：Marker Component 与类型化角色

渲染层不是一串散落的数字。用名字表达角色，读代码的人才能知道某个过滤器为什么存在。

## 本章唯一主题

使用零大小 Marker Component 表达可组合的渲染或游戏角色。

```rust
#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct Foreground;
```

- Marker 没有运行时业务数据，却可成为 Query 过滤、生成模板和系统边界。
- 一个实体可同时拥有 `PlayerSprite` 与 `Foreground`；这是组合，不是继承层级。
- Z 值仍是数值坐标，Marker 负责语义，不能自动替你决定绘制顺序。

## 设计用法

用 Marker 区分玩家、敌人、HUD 容器或可点选对象；数值配置留在专门组件或常量中。避免用字符串标签或一个巨大 `kind: u8` 替代可查询类型。

## 练习

为背景、角色和前景定义 3 个 Marker，并写 3 个互不干扰的 Query；再说明哪一部分仍必须由 Z 值控制。
