use bark_rs::{BarkMessage, Level, SyncBarkClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 演示消息构建的各种方式
    println!("🛠️  演示消息构建功能");

    // 方式1: 使用 BarkMessage::builder()
    let message1 = BarkMessage::builder()
        .title("Builder 模式")
        .body("使用 BarkMessage::builder() 构建")
        .level(Level::Active)
        .sound("minuet")
        .build();

    // 方式2: 使用 BarkMessage::new()
    let message2 = BarkMessage::new()
        .title("New 方法")
        .body("使用 BarkMessage::new() 构建")
        .level(Level::TimeSensitive)
        .volume(6)
        .build();

    // 方式3: 完整参数演示
    let complete_message = BarkMessage::builder()
        .title("完整参数演示")
        .subtitle("副标题")
        .body("这是一个包含所有参数的消息")
        .level(Level::Critical)
        .volume(8)
        .badge(3)
        .call(true)
        .auto_copy(false)
        .copy("自定义复制内容")
        .sound("alarm")
        .icon("https://example.com/icon.png")
        .group("演示组")
        .is_archive(true)
        .url("https://example.com")
        .action("none")
        .id("demo_message_001")
        .build();

    // 创建同步客户端来发送这些消息
    let client = SyncBarkClient::with_device_key("https://api.day.app", "QJ48vPutCAsPW2B6pE2A3a");

    println!("📤 发送消息1...");
    let response1 = client.send(&message1)?;
    println!("✅ 消息1发送成功: {}", response1.message);

    println!("📤 发送消息2...");
    let response2 = client.send(&message2)?;
    println!("✅ 消息2发送成功: {}", response2.message);

    println!("📤 发送完整消息...");
    let response3 = client.send(&complete_message)?;
    println!("✅ 完整消息发送成功: {}", response3.message);

    println!("🎉 所有消息发送完成！");

    Ok(())
}
