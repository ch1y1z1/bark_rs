//! # Bark Rust 客户端
//!
//! `bark_rs` 是一个功能完整的 Bark 推送服务 Rust 客户端库，采用清晰的模块化设计，支持所有官方 API 参数。
//!
//! ## 🏗️ 架构设计
//!
//! ### 三层架构
//! 1. **消息构建层** ([`BarkMessage`]) - 统一的消息构建，同步异步通用
//! 2. **同步客户端** ([`SyncBarkClient`]) - 专门处理同步发送，零运行时依赖
//! 3. **异步客户端** (`AsyncBarkClient`) - 专门处理异步发送，可选功能
//!
//! ### 设计优势
//! - 🧩 **模块分离**: 消息构建与发送客户端完全分离
//! - 🔄 **灵活复用**: 同一个消息可以用不同客户端发送
//! - 📦 **按需引入**: tokio 是可选依赖，只在需要异步功能时引入
//! - 🚫 **无冲突**: 同步和异步客户端各司其职，不会相互干扰
//!
//! ## 功能特性
//!
//! - 🚀 **默认同步** - 无需外部运行时，开箱即用
//! - ⚡ **可选异步** - 通过 feature 启用异步功能
//! - 🛠️ **Builder 模式** - 链式调用，易于使用
//! - 📱 **完整 API 支持** - 支持所有 Bark API 参数
//! - 🔄 **批量推送** - 支持向多个设备同时发送
//! - 🛡️ **完整错误处理** - 详细的错误类型和处理
//! - ✅ **全面测试** - 包含完整的测试用例
//!
//! ## 快速开始
//!
//! ### 同步使用（推荐）
//!
//! ```rust,no_run
//! use bark_rs::{SyncBarkClient, Level};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 创建同步客户端
//!     let client = SyncBarkClient::with_device_key("https://api.day.app", "your_device_key");
//!
//!     // 发送推送
//!     let response = client
//!         .message()
//!         .title("测试标题")
//!         .body("这是一个测试消息")
//!         .send()?;
//!
//!     println!("推送成功: {}", response.message);
//!     Ok(())
//! }
//! ```
//!
//! ### 异步使用
//!
//! ```rust,no_run
//! // Cargo.toml: bark_rs = { version = "0.1.0", features = ["async"] }
//! use bark_rs::{AsyncBarkClient, Level};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AsyncBarkClient::with_device_key("https://api.day.app", "your_device_key");
//!
//!     let response = client
//!         .message()
//!         .title("异步测试")
//!         .body("这是一个异步消息")
//!         .send()
//!         .await?;
//!
//!     println!("异步推送成功: {}", response.message);
//!     Ok(())
//! }
//! ```
//!
//! ### 消息构建与发送分离
//!
//! ```rust,no_run
//! use bark_rs::{SyncBarkClient, BarkMessage, Level};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 构建消息（与发送客户端无关）
//!     let message = BarkMessage::builder()
//!         .title("独立消息")
//!         .body("消息构建与发送完全分离")
//!         .level(Level::Critical)
//!         .sound("alarm")
//!         .badge(1)
//!         .build();
//!
//!     // 创建同步客户端并发送
//!     let sync_client = SyncBarkClient::with_device_key("https://api.day.app", "your_key");
//!     let response = sync_client.send(&message)?;
//!     
//!     println!("发送成功: {}", response.message);
//!     Ok(())
//! }
//! ```
//!
//! ## 通知级别说明
//!
//! - [`Level::Critical`]: 重要警告，在静音模式下也会响铃
//! - [`Level::Active`]: 默认值，系统会立即亮屏显示通知
//! - [`Level::TimeSensitive`]: 时效性通知，可在专注状态下显示
//! - [`Level::Passive`]: 仅添加到通知列表，不会亮屏提醒
//!
//! ## Features
//!
//! - `async` - 启用异步功能和 `AsyncBarkClient`

use reqwest::Error as ReqwestError;

#[cfg(feature = "async")]
mod async_client;
mod message;
mod sync_client;

// 重新导出主要类型
pub use message::{BarkMessage, BarkMessageBuilder, BarkResponse, Level};
pub use sync_client::{SyncBarkClient, SyncBarkMessageBuilder};

#[cfg(feature = "async")]
pub use async_client::{AsyncBarkClient, AsyncBarkMessageBuilder};

// 为了保持向后兼容，提供别名
pub use sync_client::SyncBarkClient as BarkClient;

#[derive(Debug)]
pub enum BarkError {
    RequestError(ReqwestError),
    InvalidUrl,
    MissingDeviceKey,
    SerializationError(serde_json::Error),
}

impl From<ReqwestError> for BarkError {
    fn from(error: ReqwestError) -> Self {
        BarkError::RequestError(error)
    }
}

impl From<serde_json::Error> for BarkError {
    fn from(error: serde_json::Error) -> Self {
        BarkError::SerializationError(error)
    }
}

impl std::fmt::Display for BarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BarkError::RequestError(e) => write!(f, "Request error: {}", e),
            BarkError::InvalidUrl => write!(f, "Invalid URL"),
            BarkError::MissingDeviceKey => write!(f, "Missing device key"),
            BarkError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for BarkError {}

pub type Result<T> = std::result::Result<T, BarkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_as_str() {
        assert_eq!(Level::Critical.as_str(), "critical");
        assert_eq!(Level::Active.as_str(), "active");
        assert_eq!(Level::TimeSensitive.as_str(), "timeSensitive");
        assert_eq!(Level::Passive.as_str(), "passive");
    }

    #[test]
    fn test_sync_client_creation() {
        let client = SyncBarkClient::new("https://api.day.app");
        assert_eq!(client.base_url, "https://api.day.app");
        assert_eq!(client.default_device_key, None);

        let client_with_key = SyncBarkClient::with_device_key("https://api.day.app/", "test_key");
        assert_eq!(client_with_key.base_url, "https://api.day.app");
        assert_eq!(
            client_with_key.default_device_key,
            Some("test_key".to_string())
        );
    }

    #[test]
    fn test_message_builder() {
        let message = BarkMessage::builder()
            .body("test body")
            .title("test title")
            .subtitle("test subtitle")
            .level(Level::Critical)
            .volume(8)
            .badge(5)
            .call(true)
            .auto_copy(false)
            .copy("copy text")
            .sound("alarm")
            .icon("https://example.com/icon.png")
            .group("test group")
            .is_archive(true)
            .url("https://example.com")
            .action("none")
            .id("message_id")
            .delete(false)
            .build();

        assert_eq!(message.body, "test body");
        assert_eq!(message.title, Some("test title".to_string()));
        assert_eq!(message.subtitle, Some("test subtitle".to_string()));
        assert_eq!(message.level, Some(Level::Critical));
        assert_eq!(message.volume, Some(8));
        assert_eq!(message.badge, Some(5));
        assert_eq!(message.call, Some(true));
        assert_eq!(message.auto_copy, Some(false));
        assert_eq!(message.copy, Some("copy text".to_string()));
        assert_eq!(message.sound, Some("alarm".to_string()));
        assert_eq!(
            message.icon,
            Some("https://example.com/icon.png".to_string())
        );
        assert_eq!(message.group, Some("test group".to_string()));
        assert_eq!(message.is_archive, Some(true));
        assert_eq!(message.url, Some("https://example.com".to_string()));
        assert_eq!(message.action, Some("none".to_string()));
        assert_eq!(message.id, Some("message_id".to_string()));
        assert_eq!(message.delete, Some(false));
    }

    #[test]
    fn test_volume_validation() {
        let message = BarkMessage::builder().volume(5).build();
        assert_eq!(message.volume, Some(5));

        let message = BarkMessage::builder().volume(15).build();
        assert_eq!(message.volume, None);
    }

    #[cfg(feature = "async")]
    #[test]
    fn test_async_client_creation() {
        let client = AsyncBarkClient::new("https://api.day.app");
        assert_eq!(client.base_url, "https://api.day.app");
        assert_eq!(client.default_device_key, None);

        let client_with_key = AsyncBarkClient::with_device_key("https://api.day.app/", "test_key");
        assert_eq!(client_with_key.base_url, "https://api.day.app");
        assert_eq!(
            client_with_key.default_device_key,
            Some("test_key".to_string())
        );
    }
}
