# Rust in Bevy 09：有限状态机与非法状态

暂停不是给每个系统塞一个 `if`。它是游戏流程进入了另一个可命名、可调度的状态。

## 本章唯一主题

用枚举建模有限状态，并把状态门控交给 `in_state` 等 Run Condition。

```rust
#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}
```

- 枚举列出合法阶段，类型层面排除了拼错字符串或同时处于两个阶段的表示。
- 当前 `State<T>` 与待切换的 `NextState<T>` 分离，切换不会在任意语句中同步重写流程。
- Run Condition 决定系统是否运行；状态转换规则决定何时进入下一阶段。两者职责不同。

## 设计用法

状态数量少且互斥时使用 `States`；多个可组合开关可用独立资源或组件。不要把「玩家是否在地面」这类实体事实膨胀成全局流程状态。

## 练习

为 `Paused` 添加进入和退出系统，并让移动系统只在 `Playing` 时调度；验证 UI 系统可选择继续运行。
