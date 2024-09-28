use std::error::Error;

use wireman_core::{
    client::{call_unary_async, tls::TlsConfig},
    descriptor::{RequestMessage, ResponseMessage},
    ProtoDescriptor,
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let desc = ProtoDescriptor::reflect("http://localhost:50051").await?;
    let service = &desc.get_services()[0];
    let method = &desc.get_methods(service)[1];
    println!("Service: {:}", service.full_name());
    println!("Method:  {:}", method.full_name());
    //
    let mut req = desc.get_request(&method);
    req.set_address("http://localhost:50051");

    let mut req = desc.get_request(&method);
    req.set_address("http://localhost:50051");

    // let resp = do_request(&req).await?;
    // println!("\nResponse:\n{:}", resp.message.to_json()?);

    Ok(())
}

pub async fn do_request(req: &RequestMessage) -> Result<ResponseMessage> {
    let tls_config = TlsConfig::native()?;
    let resp = call_unary_async(req, Some(tls_config)).await?;
    Ok(resp)
}
