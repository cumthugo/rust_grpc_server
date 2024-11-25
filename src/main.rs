//use std::time::SystemTime;
use tonic::{transport::Server, Request, Response, Status};

pub mod h625 {
    tonic::include_proto!("helloworld");
}

use h625::greeter_server::{Greeter, GreeterServer};
use h625::{HelloReply, HelloRequest};

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = h625::HelloReply {
            message: format!(
                "Hello {}!, This is H625 Ford IVI Project",
                request.into_inner().name
            ),
        };

        Ok(Response::new(reply))
    }

    /*
    async fn say_hello_stream_reply(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<tonic::Streaming<HelloReply>>, Status> {
        let mut response_stream = Vec::new();
        let reply = h625::HelloReply {
            message: format!(
                "Hello {}!, This is H625 Ford IVI Project",
                request.into_inner().name
            ),
        };
        response_stream.push(reply);

        Ok(Response::new(tonic::Streaming::from(response_stream)))
    }
    
    async fn say_hello_bidi_stream(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<tonic::Streaming<HelloReply>>, Status> {
        let mut stream = request.into_inner();
        let mut response_stream = Vec::new();
        while let Some(request) = stream.message().await? {
            response_stream.push(HelloReply {
                message: format!(
                    "Hello {}!, This is H625 Ford IVI Project",
                    request.name
                ),
            });
        }
        Ok(Response::new(tonic::Streaming::from(response_stream)))
    }
    */
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