use std::error::Error;

use tokio_stream::StreamExt;
use wireman_core::{client::call_server_streaming, ProtoDescriptor};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let desc = ProtoDescriptor::new(
        vec!["/Users/philippreiter/Rust/wireman/example/server/streaming"],
        vec!["streaming.proto"],
    )?;

    let service = &desc.get_services()[0];
    let method = &desc.get_methods(service)[0];

    if !method.is_server_streaming() {
        println!("Method must be server streaming");
    }

    let mut request = desc.get_request(&method);
    request.set_address("http://localhost:50051");

    let response = call_server_streaming(&request, None).await?;

    let mut pinned = std::pin::pin!(response);
    let stream = pinned.as_mut().get_mut();

    while let Some(message) = stream.next().await {
        let message = message?;
        println!("message: {:?}", message.message);
        println!();
    }

    Ok(())
}
