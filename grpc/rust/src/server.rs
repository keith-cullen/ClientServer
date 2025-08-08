/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

use rustgrpc::app;
use app::app_server::{AppServer, App};
use app::{Req, Resp};

const CA_CERT_PATH: &str = "../../certs/ca.crt";
const CERT_PATH: &str = "../../certs/server.crt";
const KEY_PATH: &str = "../../certs/server.key";
const HOST: &str = "0.0.0.0";
const PORT: &str = "50052";

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl App for MyServer {
    async fn get(&self, request : Request<Req>) -> Result<Response<Resp>, Status> {
        let val = "val1";
        println!("Get Name: '{}', Value: '{}'", request.into_inner().name, val);
        let response = Resp { value: val.to_string() };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", HOST, PORT).parse()?;
    let ca_cert = std::fs::read_to_string(CA_CERT_PATH)?;
    let ca_cert = Certificate::from_pem(ca_cert);
    let cert = std::fs::read_to_string(CERT_PATH)?;
    let key = std::fs::read_to_string(KEY_PATH)?;
    let identity = Identity::from_pem(cert, key);
    let tls_config = ServerTlsConfig::new()
                     .identity(identity)
                     .client_ca_root(ca_cert);
    let app_server = MyServer::default();
    Server::builder()
        .tls_config(tls_config)?
        .add_service(AppServer::new(app_server))
        .serve(addr)
        .await?;

    Ok(())
}
