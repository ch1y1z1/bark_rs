use crate::{BarkError, BarkMessage, BarkMessageBuilder, BarkResponse, Result};
use std::collections::HashMap;

pub struct SyncBarkClient {
    client: reqwest::blocking::Client,
    pub(crate) base_url: String,
    pub(crate) default_device_key: Option<String>,
}

impl SyncBarkClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            default_device_key: None,
        }
    }

    pub fn with_device_key(base_url: &str, device_key: &str) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            default_device_key: Some(device_key.to_string()),
        }
    }

    pub fn message(&self) -> SyncBarkMessageBuilder {
        SyncBarkMessageBuilder::new(self)
    }

    pub fn send(&self, message: &BarkMessage) -> Result<BarkResponse> {
        if message.device_keys.is_some() {
            self.send_batch(message)
        } else {
            self.send_single(message)
        }
    }

    fn get_device_key(&self, message: &BarkMessage) -> Result<String> {
        if let Some(key) = &message.device_key {
            Ok(key.clone())
        } else if let Some(key) = &self.default_device_key {
            Ok(key.clone())
        } else {
            Err(BarkError::MissingDeviceKey)
        }
    }

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

    fn send_batch(&self, message: &BarkMessage) -> Result<BarkResponse> {
        let url = format!("{}/push", self.base_url);
        let payload = self.build_json_payload(message)?;

        let response = self.client.post(&url).json(&payload).send()?;
        let bark_response: BarkResponse = response.json()?;
        Ok(bark_response)
    }

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

pub struct SyncBarkMessageBuilder<'a> {
    client: &'a SyncBarkClient,
    builder: BarkMessageBuilder,
}

impl<'a> SyncBarkMessageBuilder<'a> {
    fn new(client: &'a SyncBarkClient) -> Self {
        Self {
            client,
            builder: BarkMessageBuilder::new(),
        }
    }

    pub fn body(mut self, body: &str) -> Self {
        self.builder = self.builder.body(body);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.builder = self.builder.title(title);
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.builder = self.builder.subtitle(subtitle);
        self
    }

    pub fn device_key(mut self, device_key: &str) -> Self {
        self.builder = self.builder.device_key(device_key);
        self
    }

    pub fn device_keys(mut self, device_keys: Vec<String>) -> Self {
        self.builder = self.builder.device_keys(device_keys);
        self
    }

    pub fn level(mut self, level: crate::Level) -> Self {
        self.builder = self.builder.level(level);
        self
    }

    pub fn volume(mut self, volume: u8) -> Self {
        self.builder = self.builder.volume(volume);
        self
    }

    pub fn badge(mut self, badge: u32) -> Self {
        self.builder = self.builder.badge(badge);
        self
    }

    pub fn call(mut self, call: bool) -> Self {
        self.builder = self.builder.call(call);
        self
    }

    pub fn auto_copy(mut self, auto_copy: bool) -> Self {
        self.builder = self.builder.auto_copy(auto_copy);
        self
    }

    pub fn copy(mut self, copy: &str) -> Self {
        self.builder = self.builder.copy(copy);
        self
    }

    pub fn sound(mut self, sound: &str) -> Self {
        self.builder = self.builder.sound(sound);
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.builder = self.builder.icon(icon);
        self
    }

    pub fn group(mut self, group: &str) -> Self {
        self.builder = self.builder.group(group);
        self
    }

    pub fn ciphertext(mut self, ciphertext: &str) -> Self {
        self.builder = self.builder.ciphertext(ciphertext);
        self
    }

    pub fn is_archive(mut self, is_archive: bool) -> Self {
        self.builder = self.builder.is_archive(is_archive);
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.builder = self.builder.url(url);
        self
    }

    pub fn action(mut self, action: &str) -> Self {
        self.builder = self.builder.action(action);
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.builder = self.builder.id(id);
        self
    }

    pub fn delete(mut self, delete: bool) -> Self {
        self.builder = self.builder.delete(delete);
        self
    }

    pub fn send(self) -> Result<BarkResponse> {
        let message = self.builder.build();
        self.client.send(&message)
    }

    pub fn build(self) -> BarkMessage {
        self.builder.build()
    }
}
