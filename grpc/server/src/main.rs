use std::pin::Pin;

use simple::client::Request as ClientRequest;
use simple::simple_server::{Simple, SimpleServer};
use simple::{Client, Server as ServerMessage};
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let route_guide = MyServer {};

    let svc = SimpleServer::new(route_guide);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

#[derive(Debug, Default)]
struct MyServer {}

#[tonic::async_trait]
impl Simple for MyServer {
    type DefaultStream =
        Pin<Box<dyn Stream<Item = Result<ServerMessage, Status>> + Send + 'static>>;

    async fn default(
        &self,
        request: Request<Streaming<Client>>,
    ) -> Result<Response<Self::DefaultStream>, Status> {
        let mut stream = request.into_inner();

        let output = async_stream::try_stream! {
            while let Some(res) = stream.next().await {
                let res = res?;
                tokio::spawn(async move {
                    if let Some(req) = res.request {
                        match req {
                            ClientRequest::Response(s) => {
                                println!("Got a message: {:?}", s);
                            }
                            _ => {

                            }
                        }
                    }
                });
                yield ServerMessage::default();
                //Need to satify the compiler
                if false {
                    yield ServerMessage::default()
                }
            };
        };
        Ok(Response::new(Box::pin(output) as Self::DefaultStream))
    }
}
pub mod simple {
    tonic::include_proto!("simple"); // The string specified here must match the proto package name
}
