/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic::Request;

use rustgrpc::app;
use app::app_client::AppClient;
use app::Req;

const ROOT_SERVER_CERT: &str = "../../certs/root_server_cert.pem";
const CLIENT_CERT: &str = "../../certs/client_cert.pem";
const CLIENT_PRIVKEY: &str = "../../certs/client_privkey.pem";
const HOST: &str = "localhost";
const URI: &str = "http://localhost:12345";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_server_cert = std::fs::read_to_string(ROOT_SERVER_CERT)?;
    let root_server_cert = Certificate::from_pem(root_server_cert);
    let client_cert = std::fs::read_to_string(CLIENT_CERT)?;
    let client_privkey = std::fs::read_to_string(CLIENT_PRIVKEY)?;
    let client_identity = Identity::from_pem(client_cert, client_privkey);
    let tls_config = ClientTlsConfig::new()
                     .identity(client_identity)
                     .ca_certificate(root_server_cert)
                     .domain_name(HOST);
    let channel = Channel::from_static(URI)
                  .tls_config(tls_config)?
                  .connect()
                  .await?;
    let mut client  = AppClient::new(channel);
    let name = "key1";
    let request = Request::new(Req { name: name.to_string() });
    let response = client.get(request).await?;
    println!("Get Name: '{}', Value: '{}'", name, response.into_inner().value);

    Ok(())
}
