use std::time::{SystemTime, UNIX_EPOCH};
use torresix::torre_client::TorreClient;
use torresix::TorreRequest;

pub mod torresix {
    tonic::include_proto!("torresix");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TorreClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TorreRequest {
        model: 0,
        data: std::fs::read("example/image.jpg")?
    });

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let response = client.torre_predict(request).await?;
    println!("Response with {:?} in {}ms", response.into_inner().message, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()-start);

    Ok(())
}