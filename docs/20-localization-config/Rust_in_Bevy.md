# Rust in Bevy 20：Serde 边界与版本化数据

配置文件是外部输入，不是可信的 Rust 值。把它直接散进游戏规则，格式一变就会让错误横跨整个项目。

## 本章唯一主题

在 I/O 边界反序列化，在领域边界验证，并为持久格式保留版本。

```rust
#[derive(serde::Deserialize)]
struct RawBalance {
    player_speed: f32,
}

fn validate(raw: RawBalance) -> Result<Balance, String> {
    (raw.player_speed.is_finite() && raw.player_speed > 0.0)
        .then_some(Balance { player_speed: raw.player_speed })
        .ok_or_else(|| "player_speed 必须是正的有限数".into())
}
```

- `Deserialize` 只说明格式可读，不说明数值符合玩法约束。
- Raw 类型与运行时类型分开，防止无效配置进入系统。
- 存档或配置需要演进时，显式保存 `version` 并编写迁移路径。

## 设计用法

文本本地化通过稳定 key 查询，运行时 UI 只接收已解析文本。配置加载失败时给出路径和字段上下文，并选择默认值、阻止启动或降级模式。

## 练习

为配置加入 `version`，拒绝未知未来版本；再为缺少的旧字段提供明确默认值。
