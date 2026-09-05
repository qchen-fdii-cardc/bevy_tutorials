# Rust in Bevy 17：关联类型、`Result` 与异步加载接口

资产加载的难点不在「读到一个文件」，而在失败和完成都发生在未来。Trait 的关联类型把资产、配置和错误的关系写进接口。

## 本章唯一主题

阅读 `AssetLoader` 风格的 Trait 实现：关联类型定义契约，`Result` 把解析失败交给调用链处理。

```rust
impl AssetLoader for HeroLoader {
    type Asset = HeroAsset;
    type Settings = ();
    type Error = HeroLoadError;
    // async fn load(...) -> Result<Self::Asset, Self::Error>
}
```

- `Self::Asset` 等类型与 loader 绑定，调用方无需额外泛型参数猜测返回物。
- 自定义错误用 `From<std::io::Error>` 保留底层错误，`?` 负责向上传播。
- `Handle<T>` 是类型化引用，资源可尚未完成加载；使用前检查加载状态或查询 `Assets<T>`。

## 设计用法

加载请求、加载状态和已解析内容分开存放。不要把 `unwrap` 放进 loader，也不要假设 `load` 返回后资产已经可用。

## 练习

为自定义资产错误实现 `Display` 与 `From<std::io::Error>`，再将一个无效字段转换为带上下文的解析错误。
