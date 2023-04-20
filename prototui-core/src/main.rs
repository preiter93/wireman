use http::Uri;
use prototui_core::{call_unary_blocking, Result};
use prototui_core::{init, ProtoDescriptor};

fn main() -> Result<()> {
    let cfg = init()?;
    // get services, methods and a request message
    let desc = ProtoDescriptor::from_config(&cfg)?;
    let services = desc.get_services();
    for service in services.clone() {
        println!("{:?}", service.name());
    }
    let service = &services[1];
    let methods = desc.get_methods(&service);
    let method = methods.last().unwrap();
    let req = desc.get_request(method);
    println!("{:?}", method.name());
    println!("{:?}", req.message_name());
    println!("{:?}", req.to_json());

    // send message to gRPC server
    let uri = Uri::from_static("http://localhost:50051");
    call_unary_blocking(&cfg, uri, &req)?;

    Ok(())
}
