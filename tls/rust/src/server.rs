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

const CA_CERT_PATH: &str = "../../certs/ca.crt";
const CERT_PATH: &str = "../../certs/server.crt";
const KEY_PATH: &str = "../../certs/server.key";
const HOST: &str = "0.0.0.0";
const PORT: &str = "12345";
const TIMEOUT: Duration = Duration::new(1, 0);

fn main() -> std::io::Result<()> {
    let addr = format!("{}:{}", HOST, PORT);

    let ca_cert_file = File::open(CA_CERT_PATH)?;
    let mut ca_cert_reader = BufReader::new(ca_cert_file);
    let ca_certs = rustls_pemfile::certs(&mut ca_cert_reader)?;
    if ca_certs.len() < 1 {
        let msg = "Root client certificate not found";
        println!("{}", msg);
        return Ok(())
    }
    let ca_cert = &ca_certs[0];
    let ca_certificate = rustls::Certificate(ca_cert.to_vec());
    let mut ca_store = rustls::RootCertStore::empty();
    ca_store.add(&ca_certificate).unwrap();
    let client_auth = rustls::server::AllowAnyAuthenticatedClient::new(ca_store);

    let cert_file = File::open(CERT_PATH)?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs = rustls_pemfile::certs(&mut cert_reader)?;
    if certs.len() < 1 {
        let msg = "Server certificate not found";
        println!("{}", msg);
        return Ok(());
    }
    let cert = &certs[0];
    let mut certificate = Vec::new();
    certificate.push(rustls::Certificate(cert.to_vec()));

    let key;
    let key_file = File::open(KEY_PATH)?;
    let mut key_reader = BufReader::new(key_file);
    match rustls_pemfile::read_one(&mut key_reader)? {
        Some(rustls_pemfile::Item::RSAKey(item)) => key = rustls::PrivateKey(item),
        Some(rustls_pemfile::Item::PKCS8Key(item)) => key = rustls::PrivateKey(item),
        Some(rustls_pemfile::Item::ECKey(item)) => key = rustls::PrivateKey(item),
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
                              .with_single_cert(certificate, key)
                              .unwrap());
    let tcp_listener = TcpListener::bind(addr)?;
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
