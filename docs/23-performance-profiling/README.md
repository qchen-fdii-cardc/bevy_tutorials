# Bevy 教程 23：性能剖析：先测量，再谈优化

本章聚焦一个关键 Bevy 机制，并将其收敛到最小可验证实验。

## 省流版

- 这部分的目标是把一个 Bevy 机制压到最小可见状态。
- 不追求炫技的工程，追求真实的状态归属和调度边界。
- 代码必须独立运行，且测试可验证状态变化。

## 运行方式

在本目录执行：

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

## 问题模型

- 这项机制属于哪一类状态？Component、Resource、Event 还是 State？
- 哪个系统真实写入状态？
- 观测边界在哪里？日志、测试和资源快照如何说明事实？

## 故障注入

1. 把规则写到错误的 Stage。
2. 误用一个“万能”结构体承载多个事实。
3. 只看视觉，不断言 World 状态。

## 本章练习

1. 把本章的最小逻辑拆成更小的 System。
2. 将一条状态改为事件或资源。
3. 增加一个测试，验证这一机制的边界行为。

## 延伸阅读

- [Bevy 官方文档](https://bevy.org/learn/)
- [Bevy `0.19` API 文档](https://docs.rs/bevy/0.19.1/bevy/)
- [Bevy 官方示例索引](https://bevy.org/examples/)

<a id="rust-in-bevy"></a>

```{include} Rust_in_Bevy.md
:heading-offset: 1
```
