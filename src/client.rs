use rustnodegrpc::stats_client::StatsClient;
use rustnodegrpc::MeanRequest;

mod rustnodegrpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = StatsClient::connect("http://127.0.0.1:9800").await?;
    let request = tonic::Request::new(MeanRequest { a: 5, b: 4 });

    let response = client.mean(request).await?.into_inner().mean;
    println!("mean: {}", response);

    Ok(())
}
