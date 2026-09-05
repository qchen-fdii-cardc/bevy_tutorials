# Rust in Bevy 10：坐标量的单位与转换边界

两个值都叫 `Vec2`，并不保证它们处在同一空间。窗口像素、Viewport 坐标和世界坐标混算时，类型相同反而更危险。

## 本章唯一主题

将空间转换写成返回 `Option` 或 `Result` 的显式边界，而非默认它总会成功。

```rust
fn cursor_world(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let cursor = window.cursor_position()?;
    camera.viewport_to_world_2d(camera_transform, cursor).ok()
}
```

- `?` 传播「没有光标位置」这一正常缺席，不需要嵌套 `if let`。
- 相机的 Viewport、投影和变换都会影响转换结果。
- 变量名保留单位，例如 `cursor_px`、`world_pos`；复杂项目可用新类型进一步隔离。

## 设计用法

把转换集中在输入边界，规则系统只接收世界坐标。缺少窗口、相机不可用、光标在 Viewport 外都应成为可处理分支，不应 `unwrap`。

## 练习

为鼠标点选函数分别处理「无光标」「转换失败」「得到世界坐标」三条路径，并显示每条路径的调试信息。
