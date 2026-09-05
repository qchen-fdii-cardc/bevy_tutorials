# Rust in Bevy 22：测试分层与可控 World

能启动窗口的测试不等于验证了规则。ECS 的优势在于 World 可以在测试里被精确构造，再只运行目标调度。

## 本章唯一主题

区分纯函数测试与最小 `App` 集成测试，并让测试断言状态而非日志。

```rust
#[test]
fn damage_never_underflows() {
    let mut health = Health(3);
    health.0 = health.0.saturating_sub(10);
    assert_eq!(health.0, 0);
}
```

- 纯函数测试最快，适合碰撞、数值与转换规则。
- 需要验证 Query、资源或系统顺序时，创建最小 `App`，只注册相关系统与数据。
- 测试名称说明规则，断言说明观察点；窗口、音频和真实时间不应成为无关依赖。

## 设计用法

每个 bug 先缩成一个失败测试，再修实现。需要可重复时间时注入时间或调用特定 schedule，别依赖机器帧率和 sleep。

## 练习

为第 16 章的规则链写一个最小 App 测试，断言护甲先于生命值生效。
