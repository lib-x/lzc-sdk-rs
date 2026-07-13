use lzc_sdk::ApiGateway;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = std::env::var("LZC_DEVICE_API_URL")?;
    let gateway = ApiGateway::connect().await?;
    let device = gateway.device_proxy(&api_url).await?;

    let token = device.get_auth_token().await?;
    println!("device token deadline: {:?}", token.deadline());

    let photo_library = device.status().photo_library().query().await?;
    println!(
        "photo library status: {} ({})",
        photo_library.state, photo_library.reason
    );

    let _device_client = device.device();
    let _network_client = device.network();
    Ok(())
}
