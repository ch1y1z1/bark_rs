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
