//use std::time::SystemTime;
use tonic::{transport::Server, Request, Response, Status};

pub mod h625 {
    tonic::include_proto!("ford.ivi.h625");
}

use h625::greeter_server::{Greeter, GreeterServer};
use h625::{FooReply, FooRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn foo(
        &self,
        request: Request<FooRequest>,
    ) -> Result<Response<FooReply>, Status> {
        let reply = h625::FooReply {
            message: format!(
                "Hello {}!, This is H625 Ford IVI Project",
                request.into_inner().name
            ),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}