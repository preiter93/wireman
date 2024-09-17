use std::error::Error;
use wireman_core::{
    client::call_unary_blocking,
    descriptor::{RequestMessage, ResponseMessage},
    ProtoDescriptor,
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let desc = ProtoDescriptor::new(
        vec!["/Users/philippreiter/Rust/wireman/example"],
        vec!["grpc_simple/debugger.proto"],
    )?;
    let service = &desc.get_services()[0];
    let method = &desc.get_methods(service)[1];
    println!("Service: {:}", service.full_name());
    println!("Method:  {:}", method.full_name());

    let mut req = desc.get_request(&method);
    req.set_address("http://localhost:50051");

    let resp = do_request(&req)?;
    println!("\nResponse:\n{:}", resp.message.to_json()?);

    Ok(())
}

pub fn do_request(req: &RequestMessage) -> Result<ResponseMessage> {
    let resp = call_unary_blocking(req)?;
    Ok(resp)
}
