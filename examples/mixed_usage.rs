use bark_rs::{BarkMessage, Level, SyncBarkClient};

#[cfg(feature = "async")]
use bark_rs::AsyncBarkClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔀 演示混合使用场景");

    // 创建一个消息，可以被不同的客户端使用
    let shared_message = BarkMessage::builder()
        .device_key("QJ48vPutCAsPW2B6pE2A3a")
        .title("共享消息")
        .body("这个消息可以被同步和异步客户端共用")
        .level(Level::Active)
        .sound("telegraph")
        .build();

    // 使用同步客户端发送
    let sync_client = SyncBarkClient::new("https://api.day.app");
    let sync_response = sync_client.send(&shared_message)?;
    println!("✅ 同步发送成功: {}", sync_response.message);

    // 如果启用了异步功能，演示异步发送
    #[cfg(feature = "async")]
    {
        println!("🔄 也可以在异步环境中使用...");
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let async_client = AsyncBarkClient::new("https://api.day.app");
            match async_client.send(&shared_message).await {
                Ok(response) => println!("✅ 异步发送成功: {}", response.message),
                Err(e) => println!("❌ 异步发送失败: {}", e),
            }
        });
    }

    #[cfg(not(feature = "async"))]
    {
        println!("ℹ️  如需异步功能，请启用 'async' feature");
    }

    // 演示消息复用
    let another_client = SyncBarkClient::with_device_key("https://api.day.app", "default_key");

    // 克隆消息并修改部分内容
    let modified_message = BarkMessage::builder()
        .title("修改后的消息")
        .body("基于原消息修改的新消息")
        .level(shared_message.level.clone().unwrap_or(Level::Active))
        .sound(
            &shared_message
                .sound
                .clone()
                .unwrap_or_else(|| "default".to_string()),
        )
        .badge(5)
        .build();

    let response = another_client.send(&modified_message)?;
    println!("✅ 修改消息发送成功: {}", response.message);

    println!("🎉 混合使用演示完成！");

    Ok(())
}
