use std::time::SystemTime;
use prost_types::Timestamp;
use tonic::{transport::Server, Request, Response, Status};

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloReply, HelloRequest};

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("helloworld");
}

// Implement the service skeleton for the "Greeter" service
// defined in the proto
#[derive(Debug, Default)]
pub struct MyGreeter {}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {

        let received_timestamp = prost_types::Timestamp::from(SystemTime::now());
        println!("Received request from: {:?}", request);

        let reply = create_reply(received_timestamp, format!("Hello {}!", request.into_inner().name).as_str());
        println!("Sent reply: {:?}", reply);

        Ok(Response::new(reply))
    }
}

pub fn create_reply(received_timestamp: Timestamp, message: &str) -> HelloReply {
    return HelloReply {
        message: message.to_owned(),
        received_timestamp: Some(received_timestamp),
        replied_timestamp: Some(prost_types::Timestamp::from(SystemTime::now())),
    };
}

// Use the tokio runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Starting gRPC Server...");
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}