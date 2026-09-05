# Rust in Bevy 08：用 Enum 表达玩家意图

键盘键位是设备细节，移动和确认才是游戏语义。让按键直接渗入规则系统，重绑键位时就会发现耦合早已扩散。

## 本章唯一主题

用 `enum` 和穷尽匹配建立设备输入到领域动作的转换。

```rust
#[derive(Clone, Copy)]
enum Action { MoveLeft, MoveRight, Pause }

fn action_from_key(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::KeyA => Some(Action::MoveLeft),
        KeyCode::KeyD => Some(Action::MoveRight),
        KeyCode::Escape => Some(Action::Pause),
        _ => None,
    }
}
```

- `Option<Action>` 明确表示大多数按键没有本游戏动作。
- `match` 强迫调用方覆盖每种 `Action`，新增动作时遗漏分支会在编译期暴露。
- `Action` 值可在无窗口测试中构造，`KeyCode` 不行。

## 设计用法

采样系统读取 `ButtonInput<KeyCode>`，写入一个短生命周期的意图；规则系统只读取意图。模拟多个方向时，使用 `Vec2` 或独立轴值，不要让键位枚举承担物理向量职责。

## 练习

给 `Action` 增加 `Confirm`，通过编译器提示补齐所有 `match` 分支，再为另一套键位映射写一个纯函数测试。
