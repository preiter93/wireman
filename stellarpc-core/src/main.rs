use stellarpc_core::ProtoDescriptor;
use stellarpc_core::Result;

fn main() -> Result<()> {
    // get services, methods and a request message
    let desc = ProtoDescriptor::new(
        // vec!["/Users/philippreiter/Rust/stellarpc/test_utils"],
        vec!["/Users/philippreiter/Rust/stellarpc/stellarpc-core"],
        vec![
            // "test_files/recursive.proto",
            "test_files/oneof.proto",
            // "grpc_simple/greeter.proto",
            // "grpc_simple/timekeeper.proto",
            // "grpc_simple/debugger.proto",
            // "grpc_simple/productfinder.proto",
        ],
    )?;
    let services = desc.get_services();
    for service in services.clone() {
        println!("{:?}", service.name());
    }
    let service = &services[0];
    let method = &desc.get_methods(&service)[0];
    let mut req = desc.get_request(&method);

    req.message_mut().apply_template();
    println!("{:?}", req.message().to_json());
    // for method in desc.get_methods(service) {
    //     println!("{:?}", method.name());
    // }
    // let method = &methods[0];
    // let mut req = desc.get_request(method);
    // println!("{:?}", method.name());
    // println!("{:?}", method);
    // println!("{:?}", req.message_name());
    // println!("{:?}", req.message().to_json());
    // println!("{:?}", req);

    // for field in req.get_message_descriptor().fields() {
    //     println!("FIELD {:?}", field.name());
    //     let value = Value::default_value_for_field(&field);
    //     println!("VALUE {:?}", value);
    //     let message = value.as_message().unwrap();
    //     println!("MESSAGE {:?}", message.to_text_format());
    //     let desc = message.descriptor();
    //     for inner in desc.fields() {
    //         println!("FIELD {:?}", inner.name());
    //         let value = Value::default_value_for_field(&inner);
    //         println!("VALUE {:?}", value);
    //     }
    // }
    // // send message to grpc server
    // req.set_address("http://localhost:50051");
    // req.insert_metadata("metadata-key", "metadata-value")
    //     .unwrap();
    // let resp = call_unary_blocking(&req)?;
    // println!("{:?}", resp.message.to_json());

    Ok(())
}
