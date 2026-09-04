# Rust in Bevy 09：状态与条件调度

## 本章 Rust 地图

| Bevy 表面 | Rust 构造 | 设计含义 |
| --- | --- | --- |
| `Component` | `struct` + derive | 表达一条事实 |
| `Resource` | `struct` + `Default` | 存储共享状态 |
| `System` | `fn` + Query 参数 | 变换输入状态 |
| `Event` | `struct` + `Event` | 记录发生过什么 |

## 核心思想

Bevy 不是靠“大量 API 拼接”来工作，而是靠清晰的状态归属和调度边界。Rust 的类型系统和借用检查器与 ECS 共同要求你在代码中说明：谁拥有状态？谁能读取？谁能修改？

这意味着你需要把一大块需求拆开：

- 用 `Component` 表示实体事实；
- 用 `Resource` 表示全局或共享状态；
- 用 `System` 表示数据变换；
- 用 `Event` 表示状态变化；
- 用 `State` / `Run Condition` 管理流程。

## 小练习

1. 把一个“万能值”拆成明确的结构体与字段。
2. 评估它属于 Component、Resource 还是 Event。
3. 为关键状态补一条测试。

## 延伸阅读

- [Rust Book：结构体](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Rust Book：测试](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Bevy 官方文档](https://bevy.org/learn/)
