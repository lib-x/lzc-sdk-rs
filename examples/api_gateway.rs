use lzc_sdk::{ApiGateway, query_service_address, with_real_uid};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gateway = ApiGateway::connect().await?;

    let mut request = Request::new(());
    if let Ok(uid) = std::env::var("LZC_REAL_UID") {
        with_real_uid(&mut request, &uid)?;
    }
    let version = gateway.version().get(request).await?.into_inner();
    println!("runtime version: {}", version.version);

    let service_address = query_service_address().await?;
    println!("service source address: {service_address}");
    Ok(())
}
