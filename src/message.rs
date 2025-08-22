//! Bark 消息构建模块
//!
//! 这个模块包含了 Bark 消息的核心数据结构和构建器，提供了构建推送消息的统一接口。
//! 消息构建与发送客户端完全分离，同一个消息可以被不同的客户端复用。
//!
//! # 示例
//!
//! ```rust
//! use bark_rs::{BarkMessage, Level};
//!
//! let message = BarkMessage::builder()
//!     .title("标题")
//!     .body("消息内容")
//!     .level(Level::Critical)
//!     .sound("alarm")
//!     .badge(1)
//!     .build();
//! ```

use serde::Deserialize;

/// 推送通知的级别
///
/// 不同级别的推送通知会有不同的显示行为和优先级。
#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    /// 重要警告级别
    ///
    /// 在静音模式下也会响铃，会突破勿扰模式显示通知
    Critical,
    
    /// 默认活跃级别
    ///
    /// 系统会立即亮屏显示通知，这是默认值
    Active,
    
    /// 时效性通知级别
    ///
    /// 可在专注状态下显示，用于时间敏感的通知
    TimeSensitive,
    
    /// 被动级别
    ///
    /// 仅添加到通知列表，不会亮屏提醒用户
    Passive,
}

impl Level {
    /// 将级别转换为 Bark API 需要的字符串格式
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Level::Critical => "critical",
            Level::Active => "active",
            Level::TimeSensitive => "timeSensitive",
            Level::Passive => "passive",
        }
    }
}

/// Bark API 响应结构
///
/// 包含了 Bark 服务器返回的响应信息
#[derive(Debug, Deserialize)]
pub struct BarkResponse {
    /// 响应状态码，200 表示成功
    pub code: i32,
    
    /// 响应消息内容
    pub message: String,
    
    /// 可选的时间戳
    pub timestamp: Option<i64>,
}

/// Bark 推送消息
///
/// 包含了所有 Bark API 支持的参数。消息构建完成后可以被不同的客户端复用。
///
/// # 示例
///
/// ```rust
/// use bark_rs::{BarkMessage, Level};
///
/// // 基本消息
/// let simple_message = BarkMessage::builder()
///     .body("Hello World")
///     .build();
///
/// // 完整参数消息
/// let full_message = BarkMessage::builder()
///     .title("重要通知")
///     .subtitle("系统警告")
///     .body("服务器负载过高")
///     .level(Level::Critical)
///     .volume(10)
///     .badge(1)
///     .call(true)
///     .auto_copy(true)
///     .sound("alarm")
///     .icon("https://example.com/icon.png")
///     .group("系统监控")
///     .is_archive(true)
///     .url("https://monitor.example.com")
///     .id("alert_001")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct BarkMessage {
    /// 推送标题
    pub title: Option<String>,
    
    /// 推送副标题
    pub subtitle: Option<String>,
    
    /// 推送内容（必需）
    pub body: String,
    
    /// 设备密钥（单个设备）
    pub device_key: Option<String>,
    
    /// 设备密钥列表（批量推送）
    pub device_keys: Option<Vec<String>>,
    
    /// 推送级别
    pub level: Option<Level>,
    
    /// 音量大小 (1-10)
    pub volume: Option<u8>,
    
    /// 应用角标数字
    pub badge: Option<u32>,
    
    /// 是否重复播放铃声
    pub call: Option<bool>,
    
    /// 是否自动复制推送内容
    pub auto_copy: Option<bool>,
    
    /// 自定义复制内容
    pub copy: Option<String>,
    
    /// 铃声名称
    pub sound: Option<String>,
    
    /// 自定义图标 URL
    pub icon: Option<String>,
    
    /// 消息分组
    pub group: Option<String>,
    
    /// 加密文本
    pub ciphertext: Option<String>,
    
    /// 是否保存到历史
    pub is_archive: Option<bool>,
    
    /// 点击跳转的 URL
    pub url: Option<String>,
    
    /// 动作类型
    pub action: Option<String>,
    
    /// 消息唯一标识
    pub id: Option<String>,
    
    /// 是否删除消息
    pub delete: Option<bool>,
}

impl BarkMessage {
    /// 创建新的消息构建器
    ///
    /// 这是 [`BarkMessage::builder()`] 的别名方法。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::BarkMessage;
    ///
    /// let message = BarkMessage::new()
    ///     .body("Hello World")
    ///     .build();
    /// ```
    pub fn new() -> BarkMessageBuilder {
        BarkMessageBuilder::new()
    }

    /// 创建新的消息构建器
    ///
    /// 推荐使用这个方法来构建消息，提供了流畅的 Builder 模式接口。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::{BarkMessage, Level};
    ///
    /// let message = BarkMessage::builder()
    ///     .title("标题")
    ///     .body("内容")
    ///     .level(Level::Critical)
    ///     .build();
    /// ```
    pub fn builder() -> BarkMessageBuilder {
        BarkMessageBuilder::new()
    }
}

impl Default for BarkMessage {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            body: String::new(),
            device_key: None,
            device_keys: None,
            level: None,
            volume: None,
            badge: None,
            call: None,
            auto_copy: None,
            copy: None,
            sound: None,
            icon: None,
            group: None,
            ciphertext: None,
            is_archive: None,
            url: None,
            action: None,
            id: None,
            delete: None,
        }
    }
}

/// Bark 消息构建器
///
/// 提供流畅的 API 来构建 [`BarkMessage`]。支持链式调用，所有参数都是可选的（除了 body）。
///
/// # 示例
///
/// ```rust
/// use bark_rs::{BarkMessage, Level};
///
/// let message = BarkMessage::builder()
///     .body("这是必需的内容")
///     .title("可选标题")
///     .level(Level::Active)
///     .volume(8)
///     .build();
/// ```
pub struct BarkMessageBuilder {
    message: BarkMessage,
}

impl BarkMessageBuilder {
    /// 创建新的消息构建器实例
    pub fn new() -> Self {
        Self {
            message: BarkMessage::default(),
        }
    }

    /// 设置推送内容（必需）
    ///
    /// 这是唯一必需的参数，其他所有参数都是可选的。
    ///
    /// # 参数
    ///
    /// * `body` - 推送消息的主要内容
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::BarkMessage;
    ///
    /// let message = BarkMessage::builder()
    ///     .body("这是推送内容")
    ///     .build();
    /// ```
    pub fn body(mut self, body: &str) -> Self {
        self.message.body = body.to_string();
        self
    }

    /// 设置推送标题
    ///
    /// # 参数
    ///
    /// * `title` - 推送通知的标题
    pub fn title(mut self, title: &str) -> Self {
        self.message.title = Some(title.to_string());
        self
    }

    /// 设置推送副标题
    ///
    /// # 参数
    ///
    /// * `subtitle` - 推送通知的副标题
    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.message.subtitle = Some(subtitle.to_string());
        self
    }

    /// 设置单个设备密钥
    ///
    /// 用于向指定设备发送推送。如果客户端已经设置了默认设备密钥，
    /// 这里设置的密钥会覆盖默认值。
    ///
    /// # 参数
    ///
    /// * `device_key` - Bark 设备密钥
    pub fn device_key(mut self, device_key: &str) -> Self {
        self.message.device_key = Some(device_key.to_string());
        self
    }

    /// 设置多个设备密钥（批量推送）
    ///
    /// 用于同时向多个设备发送相同的推送消息。
    ///
    /// # 参数
    ///
    /// * `device_keys` - 设备密钥列表
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::BarkMessage;
    ///
    /// let message = BarkMessage::builder()
    ///     .body("批量消息")
    ///     .device_keys(vec![
    ///         "key1".to_string(),
    ///         "key2".to_string(),
    ///         "key3".to_string(),
    ///     ])
    ///     .build();
    /// ```
    pub fn device_keys(mut self, device_keys: Vec<String>) -> Self {
        self.message.device_keys = Some(device_keys);
        self
    }

    /// 设置推送级别
    ///
    /// 不同级别会影响通知的显示行为和优先级。
    ///
    /// # 参数
    ///
    /// * `level` - 推送级别，参见 [`Level`] 枚举
    pub fn level(mut self, level: Level) -> Self {
        self.message.level = Some(level);
        self
    }

    /// 设置铃声音量
    ///
    /// 音量范围是 1-10，超出范围的值会被忽略。
    ///
    /// # 参数
    ///
    /// * `volume` - 音量大小 (1-10)
    pub fn volume(mut self, volume: u8) -> Self {
        if volume <= 10 {
            self.message.volume = Some(volume);
        }
        self
    }

    /// 设置应用角标数字
    ///
    /// 在应用图标上显示的数字角标。
    ///
    /// # 参数
    ///
    /// * `badge` - 角标数字
    pub fn badge(mut self, badge: u32) -> Self {
        self.message.badge = Some(badge);
        self
    }

    /// 设置是否重复播放铃声
    ///
    /// 当设置为 true 时，铃声会重复播放直到用户查看通知。
    ///
    /// # 参数
    ///
    /// * `call` - 是否重复播放
    pub fn call(mut self, call: bool) -> Self {
        self.message.call = Some(call);
        self
    }

    /// 设置是否自动复制推送内容
    ///
    /// 当设置为 true 时，推送内容会自动复制到剪贴板。
    ///
    /// # 参数
    ///
    /// * `auto_copy` - 是否自动复制
    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.message.auto_copy = Some(auto_copy);
        self
    }

    /// 设置自定义复制内容
    ///
    /// 指定要复制到剪贴板的内容，如果不设置则复制推送内容。
    ///
    /// # 参数
    ///
    /// * `copy` - 要复制的内容
    pub fn copy(mut self, copy: &str) -> Self {
        self.message.copy = Some(copy.to_string());
        self
    }

    /// 设置铃声名称
    ///
    /// 可以是系统预设铃声名称或自定义铃声名称。
    ///
    /// # 参数
    ///
    /// * `sound` - 铃声名称（如 "alarm", "bell", "default" 等）
    pub fn sound(mut self, sound: &str) -> Self {
        self.message.sound = Some(sound.to_string());
        self
    }

    /// 设置自定义图标
    ///
    /// 通知显示时使用的自定义图标 URL。
    ///
    /// # 参数
    ///
    /// * `icon` - 图标 URL 地址
    pub fn icon(mut self, icon: &str) -> Self {
        self.message.icon = Some(icon.to_string());
        self
    }

    /// 设置消息分组
    ///
    /// 相同分组的消息会被归类显示。
    ///
    /// # 参数
    ///
    /// * `group` - 分组名称
    pub fn group(mut self, group: &str) -> Self {
        self.message.group = Some(group.to_string());
        self
    }

    /// 设置加密文本
    ///
    /// 用于端到端加密的推送内容。
    ///
    /// # 参数
    ///
    /// * `ciphertext` - 加密后的文本内容
    pub fn ciphertext(mut self, ciphertext: &str) -> Self {
        self.message.ciphertext = Some(ciphertext.to_string());
        self
    }

    /// 设置是否保存到历史
    ///
    /// 当设置为 true 时，消息会被保存到历史记录中。
    ///
    /// # 参数
    ///
    /// * `is_archive` - 是否保存到历史
    pub fn is_archive(mut self, is_archive: bool) -> Self {
        self.message.is_archive = Some(is_archive);
        self
    }

    /// 设置点击跳转 URL
    ///
    /// 用户点击推送时跳转的链接地址。
    ///
    /// # 参数
    ///
    /// * `url` - 跳转的 URL 地址
    pub fn url(mut self, url: &str) -> Self {
        self.message.url = Some(url.to_string());
        self
    }

    /// 设置动作类型
    ///
    /// 指定推送的动作行为。
    ///
    /// # 参数
    ///
    /// * `action` - 动作类型（如 "none"）
    pub fn action(mut self, action: &str) -> Self {
        self.message.action = Some(action.to_string());
        self
    }

    /// 设置消息唯一标识
    ///
    /// 用于标识和管理消息的唯一 ID。
    ///
    /// # 参数
    ///
    /// * `id` - 消息的唯一标识符
    pub fn id(mut self, id: &str) -> Self {
        self.message.id = Some(id.to_string());
        self
    }

    /// 设置是否删除消息
    ///
    /// 当设置为 true 时，会删除指定 ID 的消息。
    ///
    /// # 参数
    ///
    /// * `delete` - 是否删除消息
    pub fn delete(mut self, delete: bool) -> Self {
        self.message.delete = Some(delete);
        self
    }

    /// 构建最终的消息对象
    ///
    /// 完成消息构建并返回 [`BarkMessage`] 实例。
    ///
    /// # 返回值
    ///
    /// 返回构建完成的 [`BarkMessage`]
    ///
    /// # 示例
    ///
    /// ```rust
    /// use bark_rs::BarkMessage;
    ///
    /// let message = BarkMessage::builder()
    ///     .body("Hello World")
    ///     .title("标题")
    ///     .build();
    /// ```
    pub fn build(self) -> BarkMessage {
        self.message
    }
}
