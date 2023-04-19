// use prototui_core::get_args;
// use prototui_core::init;
// use prototui_core::list_messages;
// use prototui_core::list_methods;
// use prototui_core::list_services;
// use prototui_core::ProtoRegistry;
// use prototui_core::Result;
//
// fn main() -> Result<()> {
//     let cfg = init()?;
//     let args = get_args();
//
//     let mut registry = ProtoRegistry::new(&cfg);
//
//     match args.command.as_str() {
//         "list" => match (args.service, args.method) {
//             // if the arguments contain a Service, list Methods
//             (Some(svc), None) => list_methods(&mut registry, &svc)?,
//
//             // if the arguments contin a Service and a Method, list Messages
//             (Some(svc), Some(mth)) => list_messages(&mut registry, &svc, &mth)?,
//
//             // otherwise list Services
//             _ => list_services(&mut registry)?,
//         },
//         &_ => panic!("command {} not supported", args.command),
//     }
//
//     Ok(())
// }
//
use prototui_core::client::Client;
use prototui_core::descriptor::message::ProtoMessage;
use prototui_core::Result;
use prototui_core::{init, ProtoDescriptor};
use http::Uri;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    let cfg = init()?;
    // let args = get_args();
    let desc = ProtoDescriptor::from_config(cfg)?;
    let services = desc.get_services();
    for service in services.clone() {
        println!("{:?}", service.name());
    }
    let service = &services[1];
    let methods = desc.get_methods(&service);
    let method = methods.last().unwrap();
    let request = desc.get_request(method);
    println!("{:?}", method.name());
    println!("{:?}", request.message_name());
    println!("{:?}", request.to_json());
    // let mut client = Client::new("localhost:50051").unwrap();
    // client.unary().unwrap().await;
    let rt = Runtime::new().unwrap();

    let future = async_call(request);
    let result = rt.block_on(future);
    let s = match result {
        Ok(response) => response.to_json(),
        Err(err) => err.to_string(),
    };
    println!("{:?}", s);
    Ok(())
}

async fn async_call(req: ProtoMessage) -> Result<ProtoMessage> {
    let mut client = Client::new(Uri::from_static("http://localhost:50051"))?;
    let resp = client.unary(&req).await?;
    Ok(resp)
}
