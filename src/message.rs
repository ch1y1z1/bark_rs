use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    Critical,
    Active,
    TimeSensitive,
    Passive,
}

impl Level {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Level::Critical => "critical",
            Level::Active => "active",
            Level::TimeSensitive => "timeSensitive",
            Level::Passive => "passive",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BarkResponse {
    pub code: i32,
    pub message: String,
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct BarkMessage {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub body: String,
    pub device_key: Option<String>,
    pub device_keys: Option<Vec<String>>,
    pub level: Option<Level>,
    pub volume: Option<u8>,
    pub badge: Option<u32>,
    pub call: Option<bool>,
    pub auto_copy: Option<bool>,
    pub copy: Option<String>,
    pub sound: Option<String>,
    pub icon: Option<String>,
    pub group: Option<String>,
    pub ciphertext: Option<String>,
    pub is_archive: Option<bool>,
    pub url: Option<String>,
    pub action: Option<String>,
    pub id: Option<String>,
    pub delete: Option<bool>,
}

impl BarkMessage {
    pub fn new() -> BarkMessageBuilder {
        BarkMessageBuilder::new()
    }

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

pub struct BarkMessageBuilder {
    message: BarkMessage,
}

impl BarkMessageBuilder {
    pub fn new() -> Self {
        Self {
            message: BarkMessage::default(),
        }
    }

    pub fn body(mut self, body: &str) -> Self {
        self.message.body = body.to_string();
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.message.title = Some(title.to_string());
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.message.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn device_key(mut self, device_key: &str) -> Self {
        self.message.device_key = Some(device_key.to_string());
        self
    }

    pub fn device_keys(mut self, device_keys: Vec<String>) -> Self {
        self.message.device_keys = Some(device_keys);
        self
    }

    pub fn level(mut self, level: Level) -> Self {
        self.message.level = Some(level);
        self
    }

    pub fn volume(mut self, volume: u8) -> Self {
        if volume <= 10 {
            self.message.volume = Some(volume);
        }
        self
    }

    pub fn badge(mut self, badge: u32) -> Self {
        self.message.badge = Some(badge);
        self
    }

    pub fn call(mut self, call: bool) -> Self {
        self.message.call = Some(call);
        self
    }

    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.message.auto_copy = Some(auto_copy);
        self
    }

    pub fn copy(mut self, copy: &str) -> Self {
        self.message.copy = Some(copy.to_string());
        self
    }

    pub fn sound(mut self, sound: &str) -> Self {
        self.message.sound = Some(sound.to_string());
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.message.icon = Some(icon.to_string());
        self
    }

    pub fn group(mut self, group: &str) -> Self {
        self.message.group = Some(group.to_string());
        self
    }

    pub fn ciphertext(mut self, ciphertext: &str) -> Self {
        self.message.ciphertext = Some(ciphertext.to_string());
        self
    }

    pub fn is_archive(mut self, is_archive: bool) -> Self {
        self.message.is_archive = Some(is_archive);
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.message.url = Some(url.to_string());
        self
    }

    pub fn action(mut self, action: &str) -> Self {
        self.message.action = Some(action.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.message.id = Some(id.to_string());
        self
    }

    pub fn delete(mut self, delete: bool) -> Self {
        self.message.delete = Some(delete);
        self
    }

    pub fn build(self) -> BarkMessage {
        self.message
    }
}
