# revel_cell

一个提供可变引用计数智能指针的 Rust 库，基于 `Arc` 和 `UnsafeCell` 实现。

## 特性

- **可变引用计数**: 通过 `Arc<T>` 和 `Weak<T>` 提供可变数据的引用计数管理
- **线程安全** (可选): 通过 `thread-safe` 特性启用线程安全支持
- **标准库兼容**: 提供与标准库 `Arc`/`Weak` 类似的 API

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
revel_cell = "0.1.3"
```

启用线程安全支持：

```toml
[dependencies]
revel_cell = { version = "0.1.3", features = ["thread-safe"] }
```

## 使用示例

### 基本用法

```rust
use revel_cell::{Arc, Weak};

// 创建 Arc
let arc = Arc::new(42);
println!("值: {}", arc.value());

// 修改值
arc.set_value(100);
println!("新值: {}", arc.value());

// 使用可变引用
*arc.value_mut() = 200;
println!("最终值: {}", arc.value());
```

### 克隆和引用计数

```rust
use revel_cell::Arc;

let arc1 = Arc::new(42);
let arc2 = arc1.clone();

println!("强引用计数: {}", arc1.strong_count());
println!("arc1 == arc2: {}", arc1 == arc2);
```

### Weak 引用

```rust
use revel_cell::{Arc, Weak};

let arc = Arc::new(42);
let weak = arc.downgrade();

// 尝试升级 Weak 引用
if let Some(upgraded) = weak.upgrade() {
    println!("成功升级，值: {}", upgraded.value());
} else {
    println!("Arc 已被释放");
}

// 检查是否可以升级
if weak.upgradable() {
    println!("Weak 引用仍然有效");
}
```

### 直接访问内部值

```rust
use revel_cell::Arc;

let arc = Arc::new(String::from("Hello"));

// 通过 Deref 访问
println!("{}", arc.len());

// 通过 value() 访问
println!("{}", arc.value());

// 通过 value_mut() 修改
arc.value_mut().push_str(", World!");
println!("{}", arc.value());
```

## API 文档

### Arc<T>

- `new(value: T) -> Arc<T>`: 创建新的 `Arc`
- `downgrade(&self) -> Weak<T>`: 创建弱引用
- `value(&self) -> &T`: 获取不可变引用
- `value_mut(&self) -> &mut T`: 获取可变引用（不安全但允许）
- `set_value(&self, value: T)`: 设置新值
- `strong_count(&self) -> usize`: 获取强引用计数
- `weak_count(&self) -> usize`: 获取弱引用计数
- `inner_ptr(&self) -> *const UnsafeCell<T>`: 获取内部指针
- `from_raw(ptr) -> Arc<T>`: 从原始指针创建
- `eq_weak(&self, other: &Weak<T>) -> bool`: 与 Weak 比较

### Weak<T>

- `new() -> Weak<T>`: 创建空的 `Weak`
- `upgrade(&self) -> Option<Arc<T>>`: 尝试升级为 `Arc`
- `upgradable(&self) -> bool`: 检查是否可以升级
- `get(&self) -> Option<&mut T>`: 获取可变引用（如果可用）
- `strong_count(&self) -> usize`: 获取强引用计数
- `weak_count(&self) -> usize`: 获取弱引用计数
- `inner_ptr(&self) -> *const UnsafeCell<T>`: 获取内部指针
- `from_raw(ptr) -> Weak<T>`: 从原始指针创建
- `eq_arc(&self, other: &Arc<T>) -> bool`: 与 Arc 比较

## 安全性说明

⚠️ **警告**: 此库使用 `UnsafeCell` 来允许通过不可变引用进行可变访问。虽然这在单线程环境下通常是安全的，但需要开发者确保：

1. 不会同时持有多个可变引用
2. 在多线程环境下使用 `thread-safe` 特性时，确保类型 `T` 实现了 `Send` 和 `Sync`

## 许可证

查看 [LICENSE](LICENSE) 文件了解详情。

## 仓库

- GitHub: https://github.com/aurorax-neo/revel_cell
- 文档: https://docs.rs/revel_cell

