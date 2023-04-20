use http::Uri;
use prost_reflect::{ReflectMessage, Value};
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
    let service = &services[2];
    let methods = desc.get_methods(service);
    let method = &methods[0];
    let req = desc.get_response(method);
    println!("{:?}", method.name());
    // println!("{:?}", method);
    println!("{:?}", req.message_name());
    println!("{:?}", req.to_json());
    // println!("{:?}", req);

    for field in req.get_message_descriptor().fields() {
        println!("FIELD {:?}", field.name());
        let value = Value::default_value_for_field(&field);
        println!("VALUE {:?}", value);
        let message = value.as_message().unwrap();
        println!("MESSAGE {:?}", message.to_text_format());
        let desc = message.descriptor();
        for inner in desc.fields() {
            println!("FIELD {:?}", inner.name());
            let value = Value::default_value_for_field(&inner);
            println!("VALUE {:?}", value);
        }
    }
    // send message to gRPC server
    let uri = Uri::from_static("http://localhost:50051");
    call_unary_blocking(&cfg, uri, &req)?;

    Ok(())
}
