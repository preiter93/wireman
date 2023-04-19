use http::Uri;
use prototui_core::client::Client;
use prototui_core::descriptor::message::MethodMessage;
use prototui_core::Result;
use prototui_core::{init, ProtoDescriptor};
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

async fn async_call(req: MethodMessage) -> Result<MethodMessage> {
    let mut client = Client::new(Uri::from_static("http://localhost:50051"))?;
    let resp = client.unary(&req).await?;
    Ok(resp)
}
