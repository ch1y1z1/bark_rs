use bark_rs::{BarkError, BarkMessage, SyncBarkClient};

fn main() {
    println!("🚨 演示错误处理");

    // 创建没有默认设备密钥的客户端
    let client = SyncBarkClient::new("https://api.day.app");

    // 尝试发送没有设备密钥的消息
    let message_without_key = BarkMessage::builder()
        .title("错误演示")
        .body("这个消息没有设备密钥")
        .build();

    match client.send(&message_without_key) {
        Ok(_) => println!("❌ 意外成功"),
        Err(BarkError::MissingDeviceKey) => {
            println!("✅ 正确捕获到缺少设备密钥错误");
        }
        Err(e) => println!("❓ 其他错误: {}", e),
    }

    // 演示正确的错误处理模式
    let result = client
        .message()
        .device_key("QJ48vPutCAsPW2B6pE2A3a")
        .title("正确的消息")
        .body("这个消息有设备密钥")
        .send();

    match result {
        Ok(response) => {
            println!(
                "✅ 消息发送成功: code={}, message={}",
                response.code, response.message
            );
        }
        Err(BarkError::RequestError(e)) => {
            println!("❌ 网络请求错误: {}", e);
        }
        Err(BarkError::MissingDeviceKey) => {
            println!("❌ 缺少设备密钥");
        }
        Err(BarkError::SerializationError(e)) => {
            println!("❌ 序列化错误: {}", e);
        }
        Err(BarkError::InvalidUrl) => {
            println!("❌ 无效URL");
        }
    }

    println!("🎉 错误处理演示完成！");
}
