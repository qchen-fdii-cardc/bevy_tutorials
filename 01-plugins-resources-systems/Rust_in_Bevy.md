# Rust in Bevy 01：Trait、组合与显式依赖

「Plugin 只是一个文件夹」会让项目很快长成文件名驱动的迷宫。`Plugin` 是 Rust trait 定义的功能边界。

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `impl Plugin for RuntimeDiagnosticsPlugin` | trait 实现 | 功能以统一装配协议接入 App |
| `struct RuntimeDiagnosticsPlugin;` | 零大小类型 | 无运行时配置的能力标签 |
| `init_resource::<DebugOverlay>()` | 泛型函数与 trait bound | 由类型参数选择需初始化的状态 |
| `Res<ButtonInput<KeyCode>>` | 不可变借用 | 输入采样不能被当前 System 改写 |
| `ResMut<DebugOverlay>` | 可变借用 | 开关状态有唯一修改者 |

## Trait 是扩展点，不是继承替身

```rust
struct RuntimeDiagnosticsPlugin;

impl Plugin for RuntimeDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        // 注册本功能拥有的状态和规则
    }
}
```

Rust 没有类继承树来承载 Bevy 功能。`Plugin` trait 给出一个最小协议：任何实现 `build` 的类型都能参与装配。能力通过组合加入 App，不需要让「诊断插件」成为「游戏插件」的子类。

零大小类型没有字段，也几乎不占用运行时空间；它的价值是类型身份。需要配置时，再将字段加入 `struct RuntimeDiagnosticsPlugin { initially_enabled: bool }`，并在 `build` 中把配置写入 Resource。

## 泛型初始化为什么可靠

`init_resource::<DebugOverlay>()` 的 `::<...>` 是 turbofish 语法，显式指定泛型类型。它要求 `DebugOverlay` 满足 Bevy 的 `Resource` 与 `Default` 约束。编译器在注册点检查这份契约，调用者不会在运行时才发现缺少初值。

这体现 Rust 的「让非法状态难以构造」：将约束写进类型和 trait，而不是把缺失 Resource 的检查留给每个 System。

## 组合优于万能上下文

`DebugOverlay` 与 `Heartbeat` 是两个独立 Resource，因为它们变化原因不同：一个由输入切换，一个由时间推进。把它们合成 `GameContext` 会扩大每个 System 的可变借用范围，让本可并行的规则共享一把大锁。

在 Rust 中优先让类型表达最小职责。函数参数越准确，调用者越难无意间依赖不该依赖的数据。

## 小练习

将 `RuntimeDiagnosticsPlugin` 改成含 `enabled_by_default: bool` 的配置类型。比较「插件字段只在 `build` 时读取一次」与「Resource 可被系统每帧读取」：前者适合装配配置，后者适合运行期状态。

## 延伸阅读

- [Rust Book：Trait 定义共享行为](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust API Guidelines：构造器](https://rust-lang.github.io/api-guidelines/predictability.html)
- [Bevy `Plugin` 文档](https://docs.rs/bevy/0.19.1/bevy/app/trait.Plugin.html)