/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

extern crate rustls;
extern crate rustls_pemfile;

use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::str::from_utf8;
use std::net::TcpStream;
use std::time::Duration;
use std::sync::Arc;

const ROOT_SERVER_CERT: &str = "../../certs/root_server_cert.pem";
const CLIENT_CERT: &str = "../../certs/client_cert.pem";
const CLIENT_PRIVKEY: &str = "../../certs/client_privkey.pem";
const HOST: &str = "localhost";
const PORT: &str = "12345";
const TIMEOUT: Duration = Duration::new(1, 0);
const NUM_ITER: u32 = 5;

fn main() -> std::io::Result<()> {
    let server_addr = format!("{}:{}", HOST, PORT);

    let root_server_cert_file = File::open(ROOT_SERVER_CERT)?;
    let mut root_server_cert_reader = BufReader::new(root_server_cert_file);
    let root_server_certs = rustls_pemfile::certs(&mut root_server_cert_reader)?;
    if root_server_certs.len() < 1 {
        let msg = "Root server certificate not found";
        println!("{}", msg);
        return Ok(())
    }
    let root_server_cert = &root_server_certs[0];
    let root_server_certificate = rustls::Certificate(root_server_cert.to_vec());
    let mut root_server_store = rustls::RootCertStore::empty();
    root_server_store.add(&root_server_certificate).unwrap();

    let client_cert_file = File::open(CLIENT_CERT)?;
    let mut client_cert_reader = BufReader::new(client_cert_file);
    let client_certs = rustls_pemfile::certs(&mut client_cert_reader)?;
    if client_certs.len() < 1 {
        let msg = "Client certificate not found";
        println!("{}", msg);
        return Ok(());
    }
    let client_cert = &client_certs[0];
    let mut client_certificates = Vec::new();
    client_certificates.push(rustls::Certificate(client_cert.to_vec()));

    let client_privkey;
    let client_privkey_file = File::open(CLIENT_PRIVKEY)?;
    let mut client_privkey_reader = BufReader::new(client_privkey_file);
    match rustls_pemfile::read_one(&mut client_privkey_reader)? {
        Some(rustls_pemfile::Item::RSAKey(key)) => client_privkey = rustls::PrivateKey(key),
        Some(rustls_pemfile::Item::PKCS8Key(key)) => client_privkey = rustls::PrivateKey(key),
        Some(rustls_pemfile::Item::ECKey(key)) => client_privkey = rustls::PrivateKey(key),
        Some(_) => {
            let msg = "Client private key not recognised";
            println!("{}", msg);
            return Ok(());
        }
        None => {
            let msg = "Server private key not found";
            println!("{}", msg);
            return Ok(());
        }
    }

    let tls_config = Arc::new(rustls::ClientConfig::builder()
                              .with_safe_defaults()
                              .with_root_certificates(root_server_store)
                              .with_single_cert(client_certificates, client_privkey)
                              .unwrap());
    let tcp_stream = TcpStream::connect(server_addr);
    match tcp_stream {
        Ok(tcp_stream) => {
            println!("Opened connection to {}", tcp_stream.peer_addr().unwrap());
            tcp_stream.set_read_timeout(Some(TIMEOUT)).unwrap();
            tcp_stream.set_write_timeout(Some(TIMEOUT)).unwrap();
            let tls_config = Arc::clone(&tls_config);
            handle_connection(tcp_stream, tls_config);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}

fn handle_connection(mut tcp_stream: TcpStream, tls_config: Arc<rustls::ClientConfig>) {
    let mut buffer = [0; 1024];
    let host_name = HOST.try_into().unwrap();
    let mut tls_con = rustls::ClientConnection::new(tls_config, host_name).unwrap();
    let mut tls_stream = rustls::Stream::new(&mut tls_con, &mut tcp_stream);
    for i in 0..NUM_ITER {
        println!("Sending");
        let str = format!("hello{}", i);
        let write_res = tls_stream.write(str.as_bytes());
        match write_res {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                println!("Sent: {}", str);
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
        let flush_res = tls_stream.flush();
        if let Err(e) = flush_res {
            println!("{}", e);
            return;
        }
        println!("Receiving");
        let read_res = tls_stream.read(&mut buffer);
        match read_res {
            Ok(n) => {
                if n ==0 {
                    break;
                }
                println!("Received: {}", from_utf8(&buffer[0..n]).unwrap());
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
    println!("Connection closed");
}
