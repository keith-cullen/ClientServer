/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

extern crate rustls;
extern crate rustls_pemfile;

use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::sync::Arc;
use std::thread;

const ROOT_CLIENT_CERT: &str = "../../certs/root_client_cert.pem";
const SERVER_CERT: &str = "../../certs/server_cert.pem";
const SERVER_PRIVKEY: &str = "../../certs/server_privkey.pem";
const HOST: &str = "0.0.0.0";
const PORT: &str = "12345";
const TIMEOUT: Duration = Duration::new(1, 0);

fn main() -> std::io::Result<()> {
    let server_addr = format!("{}:{}", HOST, PORT);

    let root_client_cert_file = File::open(ROOT_CLIENT_CERT)?;
    let mut root_client_cert_reader = BufReader::new(root_client_cert_file);
    let root_client_certs = rustls_pemfile::certs(&mut root_client_cert_reader)?;
    if root_client_certs.len() < 1 {
        let msg = "Root client certificate not found";
        println!("{}", msg);
        return Ok(())
    }
    let root_client_cert = &root_client_certs[0];
    let root_client_certificate = rustls::Certificate(root_client_cert.to_vec());
    let mut root_client_store = rustls::RootCertStore::empty();
    root_client_store.add(&root_client_certificate).unwrap();
    let client_auth = rustls::server::AllowAnyAuthenticatedClient::new(root_client_store);

    let server_cert_file = File::open(SERVER_CERT)?;
    let mut server_cert_reader = BufReader::new(server_cert_file);
    let server_certs = rustls_pemfile::certs(&mut server_cert_reader)?;
    if server_certs.len() < 1 {
        let msg = "Server certificate not found";
        println!("{}", msg);
        return Ok(());
    }
    let server_cert = &server_certs[0];
    let mut server_certificates = Vec::new();
    server_certificates.push(rustls::Certificate(server_cert.to_vec()));

    let server_privkey;
    let server_privkey_file = File::open(SERVER_PRIVKEY)?;
    let mut server_privkey_reader = BufReader::new(server_privkey_file);
    match rustls_pemfile::read_one(&mut server_privkey_reader)? {
        Some(rustls_pemfile::Item::RSAKey(key)) => server_privkey = rustls::PrivateKey(key),
        Some(rustls_pemfile::Item::PKCS8Key(key)) => server_privkey = rustls::PrivateKey(key),
        Some(rustls_pemfile::Item::ECKey(key)) => server_privkey = rustls::PrivateKey(key),
        Some(_) => {
            let msg = "Server private key not recognised";
            println!("{}", msg);
            return Ok(());
        }
        None => {
            let msg = "Server private key not found";
            println!("{}", msg);
            return Ok(());
        }
    }

    let tls_config = Arc::new(rustls::ServerConfig::builder()
                              .with_safe_defaults()
                              .with_client_cert_verifier(client_auth)
                              .with_single_cert(server_certificates, server_privkey)
                              .unwrap());
    let tcp_listener = TcpListener::bind(server_addr)?;
    println!("Listening");
    let mut index: u32 = 0;
    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                println!("Accepted connection from {}", tcp_stream.peer_addr().unwrap());
                tcp_stream.set_read_timeout(Some(TIMEOUT)).unwrap();
                tcp_stream.set_write_timeout(Some(TIMEOUT)).unwrap();
                let tls_config = Arc::clone(&tls_config);
                thread::spawn(move || {
                    handle_connection(index, tcp_stream, tls_config);
                });
                index = index + 1;
            }
            Err(e) => {
                println!("Warning: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(index: u32, mut tcp_stream: TcpStream, tls_config: Arc<rustls::ServerConfig>) {
    let mut buffer = [0; 1024];
    let mut tls_con = rustls::ServerConnection::new(tls_config).unwrap();
    let mut tls_stream = rustls::Stream::new(&mut tls_con, &mut tcp_stream);
    println!("<{}> Connection open", index);
    loop {
        println!("<{}> Receiving", index);
        let num;
        let read_res = tls_stream.read(&mut buffer);
        match read_res {
            Ok(n) => {
                if n == 0 {
                    println!("<{}> EOF", index);
                    break;
                }
                println!("<{}> Received: {}", index, from_utf8(&buffer[0..n]).unwrap());
                num = n;
            }
            Err(e) => {
                println!("<{}> {}", index, e);
                return;
            }
        }
        println!("<{}> Sending", index);
        let write_res = tls_stream.write(&buffer[0..num]);
        match write_res {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                println!("<{}> Sent: {}", index, from_utf8(&buffer[0..n]).unwrap());
            }
            Err(e) => {
                println!("<{}> {}", index, e);
                return;
            }
        }
        let flush_res = tls_stream.flush();
        if let Err(e) = flush_res {
            println!("<{}> {}", index, e);
            return;
        }
    }
    println!("<{}> Connection closed", index);
}
