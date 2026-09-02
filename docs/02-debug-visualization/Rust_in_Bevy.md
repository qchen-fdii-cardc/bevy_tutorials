# Rust in Bevy 02：类型化查询、迭代器与延迟副作用

「日志只是 `println!` 的升级版」会错过 Rust 在这里提供的真正能力：结构化字段、类型化数据和延迟副作用可以组成可审计的证据链。

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `Query<&Transform, With<DebugMarker>>` | 泛型、引用与 marker type | 查询的数据形状和筛选条件都进入类型系统 |
| `for transform in &markers` | `IntoIterator` | 逐项读取借用的 Component，不取得所有权 |
| `commands.spawn(...)` | Command buffer | 将 World 变更延迟到安全应用点 |
| `info!(field = value, ...)` | macro 与结构化日志 | 保留可筛选字段，而非拼接文本 |
| `Vec2` 与 `Transform` | 值类型组合 | 坐标计算可独立于渲染表现 |

## Query 的类型参数就是选择器

```rust
Query<&Transform, With<DebugMarker>>
```

第一个类型参数说明每个匹配实体借出 `&Transform`；第二个参数是 filter，说明只匹配拥有 `DebugMarker` 的实体。函数体不需要检查空指针、类型标签字符串或运行时反射，编译器已经把数据形状固定下来。

`DebugMarker` 是 marker type：空 struct 的存在本身就是分类信息。用类型分类能避开字符串拼写错误，也让过滤条件可被 Bevy 的访问分析读取。

## 迭代时为什么不移动数据

```rust
for transform in &markers {
    gizmos.circle_2d(transform.translation.truncate(), 12.0, YELLOW);
}
```

`&markers` 产生迭代器，元素是对 World 中 `Transform` 的借用。不能把 Component 从 Query 中 move 出来，这是对的：实体仍拥有它的 Component，绘制系统只获准观察。

当逻辑只需要读值时，保留引用或复制小型 `Copy` 值；只有系统确实拥有修改理由时才请求 `&mut T`。这条 Rust 习惯直接降低 ECS 调度冲突。

## Command buffer 把副作用与观察分开

`Commands` 收集 spawn 与 despawn 请求，之后在调度器安排的安全点应用。这个设计避免遍历 World 时改变同一集合，也让 System 可以先读取稳定快照，再提交变更。

因此按 `N` 后同一帧的 Query 未必看见新探针。这里没有「立即一致性」承诺；代码应把命令排队和结果观察分到正确的系统阶段。

## 小练习

把新探针位置计算抽成 `fn next_marker_position(marker_count: usize) -> Vec2`，为它添加普通 Rust 单元测试。纯函数没有 World、窗口和输入依赖，最适合测试边界值与坐标算法。

## 延伸阅读

- [Rust Book：迭代器](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust Book：宏](https://doc.rust-lang.org/book/ch20-05-macros.html)
- [Bevy `Commands` 文档](https://docs.rs/bevy/0.19.1/bevy/ecs/system/struct.Commands.html)