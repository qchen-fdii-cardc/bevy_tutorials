# Rust in Bevy 01：Trait、派生与零大小类型

Plugin 看似是一个目录组织手段，真正的接口是一份 Trait 合约。Rust 用 Trait 把「能安装到 App」这件事从具体类型中抽离出来。

## 本章唯一主题

理解 `impl Plugin for ...` 与 `#[derive(...)]`：类型声明数据，Trait 实现声明它如何参与框架。

```rust
struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}

#[derive(Resource, Default)]
struct Score(u32);
```

- `GameplayPlugin` 没有字段，是零大小类型；它只携带安装行为。
- `Plugin` 由 Bevy 定义，`build` 是项目提供的实现。
- `#[derive(Resource)]` 生成 Bevy 识别资源所需的实现；`Default` 让 `init_resource` 有明确的初始值来源。

## 设计用法

Plugin 按功能边界命名，例如 `CombatPlugin`，而不是按某个文件名命名。Trait 是扩展点，`derive` 是减少样板代码的工具；两者都不替代模块边界。

后续章节把数据拆为 Component 时，仍沿用这个判断：一个类型需要什么能力，就只派生或实现那份能力。

## 练习

新增一个只注册日志系统的零大小 Plugin，并让它与 `Score` 的初始化分属两个 Plugin。
