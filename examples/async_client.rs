#[cfg(feature = "async")]
use bark_rs::{AsyncBarkClient, BarkMessage, Level};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建异步客户端（带默认设备密钥）
    let client = AsyncBarkClient::with_device_key("https://api.day.app", "QJ48vPutCAsPW2B6pE2A3a");

    // 方式1: 使用客户端的链式调用
    let response = client
        .message()
        .title("异步推送")
        .body("这是异步客户端发送的消息")
        .level(Level::TimeSensitive)
        .volume(9)
        .send()
        .await?;

    println!(
        "异步推送成功: code={}, message={}",
        response.code, response.message
    );

    // 方式2: 先构建消息，再发送
    let message = BarkMessage::builder()
        .title("异步独立消息")
        .body("异步环境中的独立消息构建")
        .level(Level::Critical)
        .sound("bell")
        .call(true)
        .build();

    let response = client.send(&message).await?;
    println!(
        "异步独立消息发送成功: code={}, message={}",
        response.code, response.message
    );

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("This example requires the 'async' feature to be enabled.");
    println!("Run with: cargo run --features async --example async_client");
}
