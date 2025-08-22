use bark_rs::{Level, SyncBarkClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建同步客户端
    let client = SyncBarkClient::new("https://api.day.app");

    // 批量推送到多个设备
    let response = client
        .message()
        .device_keys(vec![
            "QJ48vPutCAsPW2B6pE2A3a".to_string(),
            "device_key_2".to_string(),
            "device_key_3".to_string(),
        ])
        .title("批量推送通知")
        .body("这是一个发送给多个设备的批量消息")
        .level(Level::TimeSensitive)
        .volume(7)
        .badge(1)
        .group("批量通知")
        .send()?;

    println!(
        "批量推送成功: code={}, message={}",
        response.code, response.message
    );

    Ok(())
}
