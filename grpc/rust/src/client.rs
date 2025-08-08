/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use tonic::Request;

use rustgrpc::app;
use app::app_client::AppClient;
use app::Req;

const CA_CERT_PATH: &str = "../../certs/ca.crt";
const CERT_PATH: &str = "../../certs/client.crt";
const KEY_PATH: &str = "../../certs/client.key";
const HOST: &str = "localhost";
const URI: &str = "http://localhost:50052";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ca_cert = std::fs::read_to_string(CA_CERT_PATH)?;
    let ca_cert = Certificate::from_pem(ca_cert);
    let cert = std::fs::read_to_string(CERT_PATH)?;
    let key = std::fs::read_to_string(KEY_PATH)?;
    let identity = Identity::from_pem(cert, key);
    let tls_config = ClientTlsConfig::new()
                     .identity(identity)
                     .ca_certificate(ca_cert)
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
