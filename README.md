# rust-lab

Rust 学习 Demo 项目，每个 demo 是独立的 crate，互不影响。

## 项目结构

```
rust-lab/
├── Cargo.toml              # workspace 根配置
├── README.md
├── basics/                 # 基础语法：变量、类型、函数、控制流
├── ownership/              # 所有权、借用、生命周期
├── structs-enums/          # 结构体、枚举、模式匹配
├── collections/            # Vec、HashMap、String 等集合
├── error-handling/         # Result、Option、? 操作符
├── traits-generics/        # Trait、泛型、trait bound
├── closures-iters/         # 闭包、迭代器
├── smart-pointers/         # Box、Rc、Arc、RefCell 等
├── concurrency/            # 线程、通道、Mutex
├── async-await/            # 异步编程、tokio
├── macros/                 # 声明宏、过程宏
└── unsafe-ffi/             # unsafe、FFI
```

## 运行方式

```bash
# 进入项目根目录
cd rust-lab

# 编译检查所有 crate
cargo check

# 运行指定 demo
cargo run -p basics
cargo run -p ownership
cargo run -p concurrency

# 构建所有 crate
cargo build
```

## 添加新 demo

```bash
# 1. 创建目录结构
mkdir -p my-topic/src

# 2. 编写 Cargo.toml
cat > my-topic/Cargo.toml << 'EOF'
[package]
name = "my-topic"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "my-topic"
path = "src/main.rs"
EOF

# 3. 编写 src/main.rs

# 4. 在根 Cargo.toml 的 members 数组中添加 "my-topic"
```

## 设计说明

- **Cargo Workspace**：所有 crate 共享同一个 `target/` 目录，节省磁盘空间和编译时间
- **独立依赖**：每个 crate 有独立的 `Cargo.toml`，依赖互不干扰
- **按主题组织**：方便查找和回顾特定知识点
