use std::error::Error;
use std::time::Duration;
use std::time::SystemTime;

use greeter::greeter_client::GreeterClient;
use greeter::HelloRequest;

use crate::greeter::HelloReply;

// Import the generated proto-rust file into a module
pub mod greeter {
    tonic::include_proto!("helloworld");
}

#[derive(Default, Debug, PartialEq)]
struct Durations {
    sent_duration: Duration,
    server_duration: Duration,
    received_duration: Duration,
    roundtrip_duration: Duration,
    summed_duration: Duration,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // The client wants to SayHello.
    let client_sent_timestamp = SystemTime::now();
    let response = client
        .say_hello(HelloRequest {
            name: "Mike".to_owned(),
            sent_timestamp: Some(prost_types::Timestamp::from(client_sent_timestamp)),
        })
        .await?;
    let client_response_timestamp = SystemTime::now();

    // Determine the durations for processing the request.
    let durations = get_durations(
        client_sent_timestamp,
        client_response_timestamp,
        response.into_inner(),
    )?;

    // Show the results.
    show_durations(durations);

    Ok(())
}

fn show_durations(durations: Durations) {
    println!("sent_duration: {:?}", durations.sent_duration);
    println!("server_duration: {:?}", durations.server_duration);
    println!("received_duration: {:?}", durations.received_duration);
    println!("roundtrip_duration: {:?}", durations.roundtrip_duration);
    println!("summed_duration: {:?}", durations.summed_duration);
}

fn get_durations(
    client_sent_timestamp: SystemTime,
    client_response_timestamp: SystemTime,
    reply: HelloReply,
) -> Result<Durations, Box<dyn Error>> {
    let server_received_timestamp = match SystemTime::try_from(reply.received_timestamp.unwrap()) {
        Ok(t) => t,
        Err(e) => return Err(Box::try_from(e).unwrap()),
    };

    let server_replied_timestamp = match SystemTime::try_from(reply.replied_timestamp.unwrap()) {
        Ok(t) => t,
        Err(e) => return Err(Box::try_from(e).unwrap()),
    };

    let sent_duration = match server_received_timestamp.duration_since(client_sent_timestamp) {
        Ok(d) => d,
        Err(e) => return Err(Box::try_from(e).unwrap()),
    };

    let server_duration = match server_replied_timestamp.duration_since(server_received_timestamp) {
        Ok(d) => d,
        Err(e) => return Err(Box::try_from(e).unwrap()),
    };

    let received_duration = match client_response_timestamp.duration_since(server_replied_timestamp)
    {
        Ok(d) => d,
        Err(e) => return Err(Box::try_from(e).unwrap()),
    };

    let roundtrip_duration = match client_response_timestamp.duration_since(client_sent_timestamp) {
        Ok(d) => d,
        Err(e) => return Err(Box::try_from(e).unwrap()),
    };

    let summed_duration = sent_duration + server_duration + received_duration;

    return Ok(Durations {
        sent_duration,
        server_duration,
        received_duration,
        roundtrip_duration,
        summed_duration,
    });
}
