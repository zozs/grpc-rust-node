use tonic::{transport::Server, Request, Response, Status};

use rustnodegrpc::stats_server::{Stats, StatsServer};
use rustnodegrpc::{MeanRequest, MeanResponse};

mod rustnodegrpc;

#[derive(Default)]
pub struct MyStats {}

#[tonic::async_trait]
impl Stats for MyStats {
    async fn mean(
        &self,
        request: Request<MeanRequest>,
    ) -> Result<Response<MeanResponse>, Status> {
        let r = request.into_inner();
        println!("Got a request for: {:?}", &r);

        let reply = MeanResponse {
            mean: (r.a + r.b) as f64 / 2.0,
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:9800".parse().unwrap();
    let server = MyStats::default();

    println!("StatsServer listening on {}", addr);

    Server::builder()
        .add_service(StatsServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
