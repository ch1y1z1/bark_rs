# Bark Rust 客户端

一个功能完整的 Bark 推送服务 Rust 客户端库，采用清晰的模块化设计，支持所有官方 API 参数。

## 🏗️ 架构设计

### 三层架构
1. **消息构建层** (`BarkMessage`) - 统一的消息构建，同步异步通用
2. **同步客户端** (`SyncBarkClient`) - 专门处理同步发送，零运行时依赖
3. **异步客户端** (`AsyncBarkClient`) - 专门处理异步发送，可选功能

### 设计优势
- 🧩 **模块分离**: 消息构建与发送客户端完全分离
- 🔄 **灵活复用**: 同一个消息可以用不同客户端发送
- 📦 **按需引入**: tokio 是可选依赖，只在需要异步功能时引入
- 🚫 **无冲突**: 同步和异步客户端各司其职，不会相互干扰

## 功能特性

- 🚀 **默认同步** - 无需外部运行时，开箱即用
- ⚡ **可选异步** - 通过 feature 启用异步功能
- 🛠️ **Builder 模式** - 链式调用，易于使用
- 📱 **完整 API 支持** - 支持所有 Bark API 参数
- 🔄 **批量推送** - 支持向多个设备同时发送
- 🛡️ **完整错误处理** - 详细的错误类型和处理
- ✅ **全面测试** - 包含完整的测试用例

## 安装

```toml
[dependencies]
# 只使用同步功能（默认）
bark_rs = "0.1.0"

# 启用异步功能
bark_rs = { version = "0.1.0", features = ["async"] }
```

## 快速开始

### 同步使用（推荐）

```rust
use bark_rs::{SyncBarkClient, Level};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建同步客户端
    let client = SyncBarkClient::with_device_key("https://api.day.app", "your_device_key");

    // 发送推送
    let response = client
        .message()
        .title("测试标题")
        .body("这是一个测试消息")
        .send()?;

    println!("推送成功: {}", response.message);
    Ok(())
}
```

### 异步使用

```rust
// Cargo.toml: bark_rs = { version = "0.1.0", features = ["async"] }
use bark_rs::{AsyncBarkClient, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AsyncBarkClient::with_device_key("https://api.day.app", "your_device_key");

    let response = client
        .message()
        .title("异步测试")
        .body("这是一个异步消息")
        .send()
        .await?;

    println!("异步推送成功: {}", response.message);
    Ok(())
}
```

### 消息构建与发送分离

```rust
use bark_rs::{SyncBarkClient, BarkMessage, Level};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 构建消息（与发送客户端无关）
    let message = BarkMessage::builder()
        .title("独立消息")
        .body("消息构建与发送完全分离")
        .level(Level::Critical)
        .sound("alarm")
        .badge(1)
        .build();

    // 创建同步客户端并发送
    let sync_client = SyncBarkClient::with_device_key("https://api.day.app", "your_key");
    let response = sync_client.send(&message)?;
    
    println!("发送成功: {}", response.message);
    Ok(())
}
```

## 高级功能

### 完整参数演示

```rust
use bark_rs::{SyncBarkClient, Level};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SyncBarkClient::with_device_key("https://api.day.app", "your_device_key");

    let response = client
        .message()
        .title("重要通知")
        .subtitle("系统警告")
        .body("服务器负载过高，请立即检查！")
        .level(Level::Critical)           // 重要警告级别
        .volume(10)                       // 最大音量
        .badge(1)                         // 应用角标
        .call(true)                       // 重复播放铃声
        .auto_copy(true)                  // 自动复制内容
        .sound("alarm")                   // 警报铃声
        .icon("https://example.com/alert.png") // 自定义图标
        .group("系统监控")                 // 消息分组
        .is_archive(true)                 // 保存到历史
        .url("https://monitor.example.com") // 点击跳转链接
        .id("server_alert_001")           // 消息ID
        .send()?;

    println!("推送成功: {}", response.message);
    Ok(())
}
```

### 批量推送

```rust
let response = client
    .message()
    .device_keys(vec![
        "device_key_1".to_string(),
        "device_key_2".to_string(),
        "device_key_3".to_string(),
    ])
    .title("批量通知")
    .body("这是一个发送给多个设备的消息")
    .level(Level::TimeSensitive)
    .send()?;
```

### 混合使用场景

```rust
use bark_rs::{SyncBarkClient, AsyncBarkClient, BarkMessage, Level};

// 构建一个消息
let message = BarkMessage::builder()
    .title("共享消息")
    .body("这个消息可以被不同客户端使用")
    .level(Level::Active)
    .build();

// 同步发送
let sync_client = SyncBarkClient::new("https://api.day.app");
sync_client.send(&message)?;

// 异步发送（需要 async feature）
#[cfg(feature = "async")]
{
    let async_client = AsyncBarkClient::new("https://api.day.app");
    async_client.send(&message).await?;
}
```

## API 参考

### 客户端类型

- `SyncBarkClient` - 同步客户端，默认可用
- `AsyncBarkClient` - 异步客户端，需要 `async` feature

### 消息构建

- `BarkMessage::builder()` - 创建消息构建器
- `BarkMessage::new()` - 同上，别名方法

### 支持的参数

- **基础参数**: title, subtitle, body, device_key, device_keys
- **通知级别**: level (critical/active/timeSensitive/passive)
- **音效控制**: volume, badge, call, sound
- **复制功能**: autoCopy, copy
- **外观定制**: icon, group
- **行为控制**: isArchive, url, action
- **消息管理**: id, delete
- **加密支持**: ciphertext

## 错误处理

```rust
use bark_rs::{SyncBarkClient, BarkError};

match client.message().body("测试").send() {
    Ok(response) => println!("成功: {}", response.message),
    Err(BarkError::RequestError(e)) => println!("网络错误: {}", e),
    Err(BarkError::MissingDeviceKey) => println!("缺少设备密钥"),
    Err(BarkError::InvalidUrl) => println!("无效的URL"),
    Err(BarkError::SerializationError(e)) => println!("序列化错误: {}", e),
}
```

## 运行示例

```bash
# 同步示例
cargo run --example sync_client
cargo run --example message_builder
cargo run --example batch_push
cargo run --example error_handling

# 异步示例（需要 async feature）
cargo run --features async --example async_client

# 混合使用
cargo run --features async --example mixed_usage
```

## Features

- `async` - 启用异步功能和 `AsyncBarkClient`

## 测试

```bash
# 测试同步功能
cargo test

# 测试异步功能
cargo test --features async
```

## 设计理念

这个库遵循以下设计原则：

1. **关注点分离**: 消息构建与发送逻辑完全分离
2. **按需引入**: 异步功能是可选的，不强制依赖 tokio
3. **类型安全**: 编译时确保参数正确性
4. **易于使用**: Builder 模式提供良好的开发体验
5. **灵活扩展**: 模块化设计便于未来功能扩展

## 通知级别说明

- `Level::Critical`: 重要警告，在静音模式下也会响铃
- `Level::Active`: 默认值，系统会立即亮屏显示通知
- `Level::TimeSensitive`: 时效性通知，可在专注状态下显示
- `Level::Passive`: 仅添加到通知列表，不会亮屏提醒

## 许可证

MIT License