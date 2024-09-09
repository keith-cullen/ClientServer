/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

use tonic::transport::{Certificate, Identity, Server, ServerTlsConfig};
use tonic::{Request, Response, Status};

use rustgrpc::app;
use app::app_server::{AppServer, App};
use app::{Req, Resp};

const ROOT_CLIENT_CERT: &str = "../../certs/root_client_cert.pem";
const SERVER_CERT: &str = "../../certs/server_cert.pem";
const SERVER_PRIVKEY: &str = "../../certs/server_privkey.pem";
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
    let server_addr = format!("{}:{}", HOST, PORT).parse()?;
    let root_client_cert = std::fs::read_to_string(ROOT_CLIENT_CERT)?;
    let root_client_cert = Certificate::from_pem(root_client_cert);
    let server_cert = std::fs::read_to_string(SERVER_CERT)?;
    let server_privkey = std::fs::read_to_string(SERVER_PRIVKEY)?;
    let server_identity = Identity::from_pem(server_cert, server_privkey);
    let tls_config = ServerTlsConfig::new()
                     .identity(server_identity)
                     .client_ca_root(root_client_cert);
    let app_server = MyServer::default();
    Server::builder()
        .tls_config(tls_config)?
        .add_service(AppServer::new(app_server))
        .serve(server_addr)
        .await?;

    Ok(())
}
