# Rust in Bevy 07：时间作为显式输入

帧率是渲染条件，不是游戏规则。把时间藏在常量或全局变量里，模拟就失去了可解释的步长。

## 本章唯一主题

让更新函数显式接收 `delta_seconds`，并区分逐帧 `Update` 与固定步 `FixedUpdate`。

```rust
fn integrate(
    time: Res<Time<Fixed>>,
    mut bodies: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut bodies {
        transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    }
}
```

- `Velocity` 的单位应是每秒，位移由速度乘以本次步长得出。
- 固定步可能在一帧内执行零次或多次；它不等于显示帧。
- 输入边沿在固定步中可能丢失，因此输入采样与模拟更新需要分层。

## 设计用法

将确定性规则放入固定步，将采样、相机和纯视觉反馈放入逐帧调度。固定步仍会受浮点、随机数和外部输入影响；「固定」只固定了时间间隔。

## 练习

在 30 FPS 与 144 FPS 下运行相同的 5 秒模拟，比较位置误差，并记录使用的是哪一种 `Time`。
