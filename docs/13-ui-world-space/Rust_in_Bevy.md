# Rust in Bevy 13：组合与数据所有权边界

HUD 跟着屏幕，血条跟着实体；它们都「显示信息」，却不该共享同一套位置来源。视觉相似不等于数据归属相同。

## 本章唯一主题

用组件组合表达 UI 与世界对象的不同所有权，并仅保存稳定的实体关联。

```rust
#[derive(Component)]
struct HealthBar {
    owner: Entity,
}

#[derive(Component)]
struct HudScore;
```

- `HealthBar { owner }` 保存的是关联，不复制 `Health`；显示系统从 owner 读取当前事实。
- `HudScore` 没有 world owner，它读取全局 `Score` 资源。
- `Entity` id 只是一条引用，owner 被销毁时必须定义 UI 是同步清理、隐藏还是重绑。

## 设计用法

世界空间 UI 与游戏实体同属世界变换链；屏幕 UI 属于 UI 布局树。让各自的系统写各自的 Transform 或 UI 样式，避免一个系统跨两个坐标域「顺便」修改一切。

需要生命周期联动时，使用父子关系或明确的清理系统。不要通过名称字符串在所有实体中反查 owner。

## 练习

实现一个 `HealthBar { owner }`，当 owner 查询失败时删除或隐藏它，并说明你选择的生命周期策略。
