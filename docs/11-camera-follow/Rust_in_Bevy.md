# Rust in Bevy 11：`Result`、单实体查询与失败路径

相机跟随的常见 bug 往往来自一个未经验证的假设：「目标和相机必然各有一个」。ECS 世界不会替这个假设担保。

## 本章唯一主题

使用 `single`、`get_single` 或按实体 `get` 的结果类型，显式处理查询基数。

```rust
fn follow(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    camera.translation.x = player.translation.x;
}
```

- `Single` 将「恰好一个」写进系统参数；条件不满足时系统会报告查询错误。
- 需要恢复策略时，采用普通 Query 并处理 `get_single()` 的 `Result`。
- `Without<Player>` 同时帮助 Bevy 证明两个可变访问不会落到同一实体。

## 设计用法

菜单、重生、分屏都可能暂时没有或多出相机。把这些阶段交给状态门控或可恢复的 `Result` 分支，而非把 `unwrap` 当作设计。

相机是独立实体：跟随只读取目标并写相机 Transform，不把相机组件塞进玩家。

## 练习

故意生成两个相机，观察单实体查询的失败信息；随后改为只对带 `MainCamera` 标记的相机跟随。
