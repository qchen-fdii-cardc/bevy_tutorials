# Rust in Bevy 15：Message 作为一次性领域事实

生命值改变是状态，命中则是发生过一次的事实。把两者混为一个组件字段，重复伤害与反馈丢失迟早会出现。

## 本章唯一主题

以 Bevy `0.19` 的 `Message` 表示离散领域事实，并由消费者将其归约为状态。

```rust
#[derive(Message)]
struct Damage {
    target: Entity,
    amount: u16,
}

fn apply_damage(mut messages: MessageReader<Damage>, mut health: Query<&mut Health>) {
    for hit in messages.read() {
        if let Ok(mut value) = health.get_mut(hit.target) {
            value.0 = value.0.saturating_sub(hit.amount);
        }
    }
}
```

- 每条 Message 表示一次命中；`Health` 保存命中后的持久结果。
- 多个 reader 可各自消费同一条 Message，例如规则、音效和统计。
- reader 需要按帧读取；短生命周期消息不适合作为可无限回放的日志。

## 设计用法

检测系统只产生 `Damage`，结算系统只改 `Health`，表现系统只读取命中事实。这样伤害顺序可通过调度明确，规则也能脱离渲染测试。

旧版资料常称这套 API 为 Event；本系列以 `0.19` 的 Message 命名为准。

## 练习

增加一个记录命中次数的 Message reader，确认它不会再次扣除生命值。
