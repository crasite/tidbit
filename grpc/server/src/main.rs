use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{channel, Sender};

// use simple::client::Request as ClientRequest;
use simple::simple_server::{Simple, SimpleServer};
use simple::{Client, Question, Server as ServerMessage};
use tokio_stream::{Stream, StreamExt, StreamNotifyClose};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status, Streaming};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cert = include_str!("../../../X509/server.crt");
    let key = include_str!("../../../X509/server-key");

    let identity = Identity::from_pem(cert, key);
    let addr = "[::1]:10000".parse().unwrap();

    let route_guide = MyServer {
        sender: Arc::new(Mutex::new(vec![])),
    };
    let sender_list = route_guide.sender.clone();

    let svc = SimpleServer::new(route_guide);
    tokio::spawn(async move {
        loop {
            {
                let mut senders = sender_list.lock().unwrap();
                *senders = senders.iter().filter(|x| !x.is_closed()).cloned().collect();
                info!("Senders: {:?}", senders.len());
                for sender in senders.iter() {
                    if let Err(e) = sender.try_send(format!("ls")) {
                        println!("Error: {:?}", e);
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
struct MyServer {
    sender: Arc<Mutex<Vec<Sender<String>>>>,
}

#[tonic::async_trait]
impl Simple for MyServer {
    type DefaultStream =
        Pin<Box<dyn Stream<Item = Result<ServerMessage, Status>> + Send + 'static>>;

    async fn default(
        &self,
        request: Request<Streaming<Client>>,
    ) -> Result<Response<Self::DefaultStream>, Status> {
        let hostname = request.metadata().get("hostname").cloned();
        let mut stream = request.into_inner();
        let (tx, mut rx) = channel(10);
        self.sender.lock().unwrap().push(tx);
        let command_stream = async_stream::try_stream! {
            while let Some(res) = rx.recv().await {
                yield ServerMessage {
                    question: Question::GetInfo as i32,
                    extra_text: res,
                };
            }
        };
        let output = async_stream::try_stream! {
            while let Some(res) = stream.next().await {
                let res = res?;
                let hostname = hostname.clone();
                tokio::spawn(async move {
                    if let Some(req) = res.request {
                        dbg!(&hostname,&req);
                    }
                });
                //Need to satify the compiler
                if false {
                    yield ServerMessage::default()
                }
            };
            dbg!("Done");
        };
        let st1 = StreamNotifyClose::new(command_stream);
        let st2 = StreamNotifyClose::new(output);
        let merge_stream = st1
            .merge(st2)
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap());
        Ok(Response::new(Box::pin(merge_stream) as Self::DefaultStream))
    }
}
pub mod simple {
    tonic::include_proto!("simple"); // The string specified here must match the proto package name
}
