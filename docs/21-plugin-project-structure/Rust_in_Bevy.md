# Rust in Bevy 21：模块可见性与功能边界

文件多不代表模块化；只要所有类型都 `pub`，任何代码都能越过边界修改内部状态。项目结构的收益来自可见性约束。

## 本章唯一主题

用模块、`pub(crate)` 与小型公开安装函数定义 Plugin 的功能接口。

```rust
mod combat {
    use bevy::prelude::*;

    pub(crate) struct CombatPlugin;
    #[derive(Component)]
    pub(crate) struct Health(pub(crate) u16);
}
```

- 默认私有是有用的；只公开跨模块确实需要的类型和函数。
- `pub(crate)` 允许同一 crate 的集成，避免把内部实现承诺给外部 crate。
- Plugin 是模块的安装入口，不应成为无边界的全局注册表。

## 设计用法

按功能聚合组件、系统、测试和资产接口，例如 `combat`、`ui`、`movement`。依赖方向指向稳定领域类型；UI 模块不应反向接管战斗规则。

## 练习

将一个功能拆入模块，先让组件保持私有，再只导出安装 Plugin 所必需的接口。
