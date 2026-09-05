# Rust in Bevy 02：`Debug`、断言与可观察的失败

日志不是调试结束时的装饰；它是把运行时事实带回开发者工作记忆的接口。先让值可见，再讨论抽象。

## 本章唯一主题

使用 `Debug` 格式化与断言表达可验证的不变量。

```rust
#[derive(Component, Debug)]
struct Velocity(Vec2);

fn check_speed(query: Query<&Velocity>) {
    for velocity in &query {
        debug_assert!(velocity.0.is_finite(), "{velocity:?}");
    }
}
```

- `#[derive(Debug)]` 让 `{:?}` 能打印结构字段；它不改变游戏逻辑。
- `debug_assert!` 用于开发期必须成立的条件，release 构建可能移除它。
- 玩家输入错误、文件缺失这类预期失败不能靠断言处理，应返回或记录可恢复的错误。

## 设计用法

日志记录事件和关键字段，Gizmos 记录空间关系，测试记录可重复的结果。三者回答的问题不同：不要用每帧 `println!` 代替状态检查。

为调试组件派生 `Debug` 很便宜；为敏感或体积很大的资源实现自定义输出更合适。

## 练习

为一个位置组件派生 `Debug`，在检测到非有限坐标时触发断言，并用实体数量和位置作为日志字段。
