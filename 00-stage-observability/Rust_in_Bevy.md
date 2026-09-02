# Rust in Bevy 阶段 0：派生状态、格式化与 UI 数据流

「把 FPS 文本塞进全局变量」只是在复制状态。覆盖层应从可审计的 Resource 和 Query 派生，它才能和实际 World 保持一致。

## Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `RuntimeStats` | `struct` 与 `Default` trait | 集中管理唯一运行时采样状态 |
| `Query<&mut Text, With<OverlayText>>` | 可变借用与 marker type | 仅授予 UI 文本的最小写权限 |
| `format!(...)` | 格式化 macro | 从数值状态生成显示文本 |
| `Visibility` match | 穷尽模式匹配 | 明确表达显示状态转换 |
| `Commands` | 延迟命令缓冲 | 遍历期间安全排队新增实体 |

## 派生状态不应拥有第二个真相源

```rust
text.0 = format!("fps: {:>5.0}", stats.frames_per_second);
```

UI 文本是 `RuntimeStats` 的派生表现，不是独立游戏状态。系统每帧从 Resource 读取数值，再写入 `Text` Component。这样即使 UI 被隐藏，采样仍继续；UI 恢复时无需同步两份 FPS 数据。

Rust 的所有权模型鼓励这种单一真相源：拥有状态的 Resource 管理值，读取者借用它，表现层只保存可重建结果。

## `match` 让状态转换完整可见

```rust
*visibility = match *visibility {
    Visibility::Hidden => Visibility::Inherited,
    _ => Visibility::Hidden,
};
```

模式匹配把隐藏状态和其他可见状态的转换集中在一个表达式中。若未来需要区分 `Visible`、`Inherited` 与 `Hidden`，应写出每条规则，而不是让布尔变量在多个系统中漂移。

## 格式化与类型检查

`format!` 在编译时检查格式字符串和参数类型。`{:>5.2}` 指定右对齐、最小宽度与两位小数；它让帧时间在数值波动时保持列对齐，调试读数不随内容跳动。

不要在热路径中用格式化文本参与游戏规则。这里每帧创建字符串只服务于开发期调试 UI；性能敏感的运行期界面应按测量结果设计更新频率。

## 小练习

为 `RuntimeStats` 添加 `fn should_report(&self) -> bool`，将「整秒采样完成」的判断封装成纯方法。再为它写普通单元测试，验证时间从 `0.99` 增至 `1.01` 时只触发一次报告。

## 延伸阅读

- [Rust Book：结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust Book：模式与匹配](https://doc.rust-lang.org/book/ch19-00-patterns.html)
- [Rust 文档：`format!`](https://doc.rust-lang.org/std/macro.format.html)