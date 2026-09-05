# Rust in Bevy 18：局部状态、Timer 与封装

动画帧号是时间演化的局部状态，不是全局游戏流程。把它提升为 Resource 会让所有精灵争抢同一个时钟。

## 本章唯一主题

用 Component 封装每个实体自己的 `Timer`、帧索引和播放策略。

```rust
#[derive(Component)]
struct Animation {
    timer: Timer,
    frame: usize,
    frames: std::ops::Range<usize>,
}
```

- 每个实体各有一份 `Animation`，因此速度、循环区间和暂停状态可不同。
- 系统借用 `&mut Animation` 推进内部状态，并在边界处重置帧号。
- 把帧推进封装为方法，可集中处理空帧区间、循环与停止等不变量。

## 设计用法

游戏状态决定动画「应该呈现什么」，动画组件决定「何时切下一帧」。这条分界避免渲染帧率直接改变规则状态。

局部状态只在没有实体归属时才考虑 `Local<T>`；多实体共享的 `Local` 不是 Component 的替代品。

## 练习

为 `Animation` 增加 `advance(&mut self, delta: Duration)`，测试循环动画在最后一帧后回到起点。
