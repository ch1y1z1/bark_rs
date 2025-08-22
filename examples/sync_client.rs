use bark_rs::{BarkMessage, Level, SyncBarkClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建同步客户端（带默认设备密钥）
    let client = SyncBarkClient::with_device_key("https://api.day.app", "QJ48vPutCAsPW2B6pE2A3a");

    // 方式1: 使用客户端的链式调用
    let response = client
        .message()
        .title("同步推送")
        .body("这是同步客户端发送的消息")
        .level(Level::Active)
        .volume(7)
        .send()?;

    println!(
        "同步推送成功: code={}, message={}",
        response.code, response.message
    );

    // 方式2: 先构建消息，再发送
    let message = BarkMessage::builder()
        .title("独立构建的消息")
        .body("消息构建与发送分离")
        .level(Level::Critical)
        .sound("alarm")
        .badge(1)
        .build();

    let response = client.send(&message)?;
    println!(
        "独立消息发送成功: code={}, message={}",
        response.code, response.message
    );

    Ok(())
}
