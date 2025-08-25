//! 同步 Bark 客户端模块
//!
//! 这个模块提供了同步的 Bark 推送客户端实现，使用 reqwest 的 blocking 客户端。
//! 同步客户端没有任何异步依赖，可以在任何环境中使用，包括异步运行时内部。
//!
//! # 特性
//!
//! - 零异步依赖，在任何环境下都能工作
//! - 支持单个设备和批量推送
//! - 提供 Builder 模式的流畅 API
//! - 完整的错误处理
//!
//! # 示例
//!
//! ```rust,no_run
//! use bark_rs::{SyncBarkClient, Level};
//!
//! let client = SyncBarkClient::with_device_key("https://api.day.app", "your_key");
//!
//! let response = client
//!     .message()
//!     .title("测试标题")
//!     .body("测试内容")
//!     .level(Level::Critical)
//!     .send()?;
//!
//! println!("发送成功: {}", response.message);
//! # Ok::<(), bark_rs::BarkError>(())
//! ```

use crate::{BarkError, BarkMessage, BarkMessageBuilder, BarkResponse, Result};
use std::collections::HashMap;

/// 同步 Bark 推送客户端
///
/// 使用 reqwest 的 blocking 客户端实现，无需异步运行时，可以在任何环境下工作。
/// 支持单个设备推送和批量推送功能。
///
/// # 创建客户端
///
/// ```rust,no_run
/// use bark_rs::SyncBarkClient;
///
/// // 创建没有默认设备密钥的客户端
/// let client = SyncBarkClient::new("https://api.day.app");
///
/// // 创建带有默认设备密钥的客户端
/// let client = SyncBarkClient::with_device_key("https://api.day.app", "your_device_key");
/// ```
///
/// # 发送消息
///
/// ```rust,no_run
/// use bark_rs::{SyncBarkClient, BarkMessage, Level};
///
/// let client = SyncBarkClient::with_device_key("https://api.day.app", "your_key");
///
/// // 方式 1: 使用 message() 方法的 Builder 模式
/// let response = client
///     .message()
///     .title("标题")
///     .body("内容")
///     .level(Level::Active)
///     .send()?;
///
/// // 方式 2: 先构建消息再发送
/// let message = BarkMessage::builder()
///     .title("标题")
///     .body("内容")
///     .build();
///
/// let response = client.send(&message)?;
/// # Ok::<(), bark_rs::BarkError>(())
/// ```
pub struct SyncBarkClient {
    /// 内部 HTTP 客户端
    client: reqwest::blocking::Client,

    /// Bark 服务器的基础 URL
    pub(crate) base_url: String,

    /// 可选的默认设备密钥
    pub(crate) default_device_key: Option<String>,
}

impl SyncBarkClient {
    /// 创建新的同步 Bark 客户端
    ///
    /// 创建一个没有默认设备密钥的客户端实例。发送消息时需要在消息中指定设备密钥，
    /// 或者使用 [`SyncBarkClient::with_device_key`] 创建带默认密钥的客户端。
    ///
    /// # 参数
    ///
    /// * `base_url` - Bark 服务器的基础 URL（如 `https://api.day.app`）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::SyncBarkClient;
    ///
    /// let client = SyncBarkClient::new("https://api.day.app");
    /// ```
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            default_device_key: None,
        }
    }

    /// 创建带有默认设备密钥的同步 Bark 客户端
    ///
    /// 创建一个具有默认设备密钥的客户端实例。如果消息中没有指定设备密钥，
    /// 将使用这里设置的默认密钥。消息中的密钥设置会覆盖默认密钥。
    ///
    /// # 参数
    ///
    /// * `base_url` - Bark 服务器的基础 URL
    /// * `device_key` - 默认的设备密钥
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::SyncBarkClient;
    ///
    /// let client = SyncBarkClient::with_device_key(
    ///     "https://api.day.app",
    ///     "your_device_key"
    /// );
    /// ```
    pub fn with_device_key(base_url: &str, device_key: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            default_device_key: Some(device_key.to_string()),
        }
    }

    /// 创建消息构建器
    ///
    /// 返回一个与此客户端关联的消息构建器，支持链式调用来构建和发送消息。
    ///
    /// # 返回值
    ///
    /// 返回 [`SyncBarkMessageBuilder`] 实例
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use bark_rs::{SyncBarkClient, Level};
    ///
    /// let client = SyncBarkClient::with_device_key("https://api.day.app", "key");
    ///
    /// let response = client
    ///     .message()
    ///     .title("标题")
    ///     .body("内容")
    ///     .level(Level::Active)
    ///     .send()?;
    /// # Ok::<(), bark_rs::BarkError>(())
    /// ```
    pub fn message(&self) -> SyncBarkMessageBuilder {
        SyncBarkMessageBuilder::new(self)
    }

    /// 发送 Bark 推送消息
    ///
    /// 根据消息是否包含多个设备密钥自动选择单个发送或批量发送。
    /// 如果消息和客户端都没有设备密钥，将返回错误。
    ///
    /// # 参数
    ///
    /// * `message` - 要发送的消息
    ///
    /// # 返回值
    ///
    /// 成功时返回 [`BarkResponse`]，失败时返回 [`BarkError`]
    ///
    /// # 错误
    ///
    /// * [`BarkError::MissingDeviceKey`] - 缺少设备密钥
    /// * [`BarkError::RequestError`] - 网络请求错误
    /// * [`BarkError::SerializationError`] - 序列化错误
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use bark_rs::{SyncBarkClient, BarkMessage};
    ///
    /// let client = SyncBarkClient::with_device_key("https://api.day.app", "key");
    /// let message = BarkMessage::builder()
    ///     .body("测试消息")
    ///     .build();
    ///
    /// let response = client.send(&message)?;
    /// println!("发送成功: {}", response.message);
    /// # Ok::<(), bark_rs::BarkError>(())
    /// ```
    pub fn send(&self, message: &BarkMessage) -> Result<BarkResponse> {
        if message.device_keys.is_some() {
            self.send_batch(message)
        } else {
            self.send_single(message)
        }
    }

    /// 获取有效的设备密钥
    ///
    /// 优先使用消息中的设备密钥，如果没有则使用客户端的默认密钥。
    /// 如果都没有，则返回错误。
    fn get_device_key(&self, message: &BarkMessage) -> Result<String> {
        if let Some(key) = &message.device_key {
            Ok(key.clone())
        } else if let Some(key) = &self.default_device_key {
            Ok(key.clone())
        } else {
            Err(BarkError::MissingDeviceKey)
        }
    }

    /// 发送单个设备的推送消息
    fn send_single(&self, message: &BarkMessage) -> Result<BarkResponse> {
        let device_key = self.get_device_key(message)?;
        let url = format!("{}/push", self.base_url);

        let mut payload = self.build_json_payload(message)?;
        payload.insert(
            "device_key".to_string(),
            serde_json::Value::String(device_key),
        );

        let response = self.client.post(&url).json(&payload).send()?;
        let bark_response: BarkResponse = response.json()?;
        Ok(bark_response)
    }

    /// 发送批量推送消息（多个设备）
    fn send_batch(&self, message: &BarkMessage) -> Result<BarkResponse> {
        let url = format!("{}/push", self.base_url);
        let payload = self.build_json_payload(message)?;

        let response = self.client.post(&url).json(&payload).send()?;
        let bark_response: BarkResponse = response.json()?;
        Ok(bark_response)
    }

    /// 构建发送给 Bark API 的 JSON 负载
    ///
    /// 将 BarkMessage 转换为 Bark API 期望的 JSON 格式
    fn build_json_payload(
        &self,
        message: &BarkMessage,
    ) -> Result<HashMap<String, serde_json::Value>> {
        let mut payload = HashMap::new();

        payload.insert(
            "body".to_string(),
            serde_json::Value::String(message.body.clone()),
        );

        if let Some(title) = &message.title {
            payload.insert(
                "title".to_string(),
                serde_json::Value::String(title.clone()),
            );
        }

        if let Some(subtitle) = &message.subtitle {
            payload.insert(
                "subtitle".to_string(),
                serde_json::Value::String(subtitle.clone()),
            );
        }

        if let Some(device_keys) = &message.device_keys {
            payload.insert(
                "device_keys".to_string(),
                serde_json::to_value(device_keys)?,
            );
        }

        if let Some(level) = &message.level {
            payload.insert(
                "level".to_string(),
                serde_json::Value::String(level.as_str().to_string()),
            );
        }

        if let Some(volume) = message.volume {
            if volume <= 10 {
                payload.insert(
                    "volume".to_string(),
                    serde_json::Value::Number(volume.into()),
                );
            }
        }

        if let Some(badge) = message.badge {
            payload.insert("badge".to_string(), serde_json::Value::Number(badge.into()));
        }

        if let Some(call) = message.call {
            payload.insert(
                "call".to_string(),
                serde_json::Value::String(if call { "1" } else { "0" }.to_string()),
            );
        }

        if let Some(auto_copy) = message.auto_copy {
            payload.insert(
                "autoCopy".to_string(),
                serde_json::Value::String(if auto_copy { "1" } else { "0" }.to_string()),
            );
        }

        if let Some(copy) = &message.copy {
            payload.insert("copy".to_string(), serde_json::Value::String(copy.clone()));
        }

        if let Some(sound) = &message.sound {
            payload.insert(
                "sound".to_string(),
                serde_json::Value::String(sound.clone()),
            );
        }

        if let Some(icon) = &message.icon {
            payload.insert("icon".to_string(), serde_json::Value::String(icon.clone()));
        }

        if let Some(group) = &message.group {
            payload.insert(
                "group".to_string(),
                serde_json::Value::String(group.clone()),
            );
        }

        if let Some(ciphertext) = &message.ciphertext {
            payload.insert(
                "ciphertext".to_string(),
                serde_json::Value::String(ciphertext.clone()),
            );
        }

        if let Some(is_archive) = message.is_archive {
            payload.insert(
                "isArchive".to_string(),
                serde_json::Value::String(if is_archive { "1" } else { "0" }.to_string()),
            );
        }

        if let Some(url) = &message.url {
            payload.insert("url".to_string(), serde_json::Value::String(url.clone()));
        }

        if let Some(action) = &message.action {
            payload.insert(
                "action".to_string(),
                serde_json::Value::String(action.clone()),
            );
        }

        if let Some(id) = &message.id {
            payload.insert("id".to_string(), serde_json::Value::String(id.clone()));
        }

        if let Some(delete) = message.delete {
            payload.insert(
                "delete".to_string(),
                serde_json::Value::String(if delete { "1" } else { "0" }.to_string()),
            );
        }

        Ok(payload)
    }
}

/// 同步 Bark 消息构建器
///
/// 与 [`SyncBarkClient`] 关联的消息构建器，提供流畅的 API 来构建和直接发送消息。
/// 它包装了通用的 [`BarkMessageBuilder`] 并添加了 [`send()`](Self::send) 方法。
///
/// # 特性
///
/// - 支持所有 [`BarkMessageBuilder`] 的方法
/// - 提供 [`send()`](Self::send) 方法直接发送消息
/// - 支持 [`build()`](Self::build) 方法构建消息对象
///
/// # 示例
///
/// ```rust,no_run
/// use bark_rs::{SyncBarkClient, Level};
///
/// let client = SyncBarkClient::with_device_key("https://api.day.app", "key");
///
/// // 构建并发送
/// let response = client
///     .message()
///     .title("标题")
///     .body("内容")
///     .level(Level::Active)
///     .send()?;
///
/// // 或者只构建
/// let message = client
///     .message()
///     .body("内容")
///     .build();
/// # Ok::<(), bark_rs::BarkError>(())
/// ```
pub struct SyncBarkMessageBuilder<'a> {
    /// 关联的同步客户端
    client: &'a SyncBarkClient,
    /// 内部的消息构建器
    builder: BarkMessageBuilder,
}

impl<'a> SyncBarkMessageBuilder<'a> {
    /// 创建新的同步消息构建器实例
    fn new(client: &'a SyncBarkClient) -> Self {
        Self {
            client,
            builder: BarkMessageBuilder::new(),
        }
    }

    /// 设置推送内容（必需）
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::body`]。
    pub fn body(mut self, body: &str) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    /// 设置推送标题
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::title`]。
    pub fn title(mut self, title: &str) -> Self {
        self.builder = self.builder.title(title);
        self
    }

    /// 设置推送副标题
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::subtitle`]。
    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.builder = self.builder.subtitle(subtitle);
        self
    }

    /// 设置单个设备密钥
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::device_key`]。
    pub fn device_key(mut self, device_key: &str) -> Self {
        self.builder = self.builder.device_key(device_key);
        self
    }

    /// 设置多个设备密钥（批量推送）
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::device_keys`]。
    pub fn device_keys(mut self, device_keys: Vec<String>) -> Self {
        self.builder = self.builder.device_keys(device_keys);
        self
    }

    /// 设置推送级别
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::level`]。
    pub fn level(mut self, level: crate::Level) -> Self {
        self.builder = self.builder.level(level);
        self
    }

    /// 设置铃声音量 (1-10)
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::volume`]。
    pub fn volume(mut self, volume: u8) -> Self {
        self.builder = self.builder.volume(volume);
        self
    }

    /// 设置应用角标数字
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::badge`]。
    pub fn badge(mut self, badge: u32) -> Self {
        self.builder = self.builder.badge(badge);
        self
    }

    /// 设置是否重复播放铃声
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::call`]。
    pub fn call(mut self, call: bool) -> Self {
        self.builder = self.builder.call(call);
        self
    }

    /// 设置是否自动复制推送内容
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::auto_copy`]。
    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.builder = self.builder.auto_copy(auto_copy);
        self
    }

    /// 设置自定义复制内容
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::copy`]。
    pub fn copy(mut self, copy: &str) -> Self {
        self.builder = self.builder.copy(copy);
        self
    }

    /// 设置铃声名称
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::sound`]。
    pub fn sound(mut self, sound: &str) -> Self {
        self.builder = self.builder.sound(sound);
        self
    }

    /// 设置自定义图标
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::icon`]。
    pub fn icon(mut self, icon: &str) -> Self {
        self.builder = self.builder.icon(icon);
        self
    }

    /// 设置消息分组
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::group`]。
    pub fn group(mut self, group: &str) -> Self {
        self.builder = self.builder.group(group);
        self
    }

    /// 设置加密文本
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::ciphertext`]。
    pub fn ciphertext(mut self, ciphertext: &str) -> Self {
        self.builder = self.builder.ciphertext(ciphertext);
        self
    }

    /// 设置是否保存到历史
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::is_archive`]。
    pub fn is_archive(mut self, is_archive: bool) -> Self {
        self.builder = self.builder.is_archive(is_archive);
        self
    }

    /// 设置点击跳转 URL
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::url`]。
    pub fn url(mut self, url: &str) -> Self {
        self.builder = self.builder.url(url);
        self
    }

    /// 设置动作类型
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::action`]。
    pub fn action(mut self, action: &str) -> Self {
        self.builder = self.builder.action(action);
        self
    }

    /// 设置消息唯一标识
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::id`]。
    pub fn id(mut self, id: &str) -> Self {
        self.builder = self.builder.id(id);
        self
    }

    /// 设置是否删除消息
    ///
    /// 详细说明请参见 [`BarkMessageBuilder::delete`]。
    pub fn delete(mut self, delete: bool) -> Self {
        self.builder = self.builder.delete(delete);
        self
    }

    /// 构建并立即发送消息
    ///
    /// 这是一个便捷方法，相当于先调用 [`build()`](Self::build) 再调用 [`SyncBarkClient::send`]。
    ///
    /// # 返回值
    ///
    /// 成功时返回 [`BarkResponse`]，失败时返回 [`BarkError`]
    ///
    /// # 错误
    ///
    /// 可能返回的错误类型与 [`SyncBarkClient::send`] 相同。
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use bark_rs::{SyncBarkClient, Level};
    ///
    /// let client = SyncBarkClient::with_device_key("https://api.day.app", "key");
    ///
    /// let response = client
    ///     .message()
    ///     .body("测试消息")
    ///     .title("测试")
    ///     .level(Level::Active)
    ///     .send()?;
    ///
    /// println!("发送成功: {}", response.message);
    /// # Ok::<(), bark_rs::BarkError>(())
    /// ```
    pub fn send(self) -> Result<BarkResponse> {
        let message = self.builder.build();
        self.client.send(&message)
    }

    /// 构建消息对象而不发送
    ///
    /// 如果您需要先构建消息再由其他客户端发送，或者需要复用消息，可以使用这个方法。
    ///
    /// # 返回值
    ///
    /// 返回构建完成的 [`BarkMessage`]
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::SyncBarkClient;
    ///
    /// let client = SyncBarkClient::new("https://api.day.app");
    ///
    /// let message = client
    ///     .message()
    ///     .body("消息内容")
    ///     .title("消息标题")
    ///     .build();
    ///
    /// // 现在可以用不同的客户端发送这个消息
    /// ```
    pub fn build(self) -> BarkMessage {
        self.builder.build()
    }
}
