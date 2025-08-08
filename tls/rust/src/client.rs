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

const CA_CERT_PATH: &str = "../../certs/ca.crt";
const CERT_PATH: &str = "../../certs/client.crt";
const KEY_PATH: &str = "../../certs/client.key";
const HOST: &str = "localhost";
const PORT: &str = "12345";
const TIMEOUT: Duration = Duration::new(1, 0);
const NUM_ITER: u32 = 5;

fn main() -> std::io::Result<()> {
    let addr = format!("{}:{}", HOST, PORT);

    let ca_cert_file = File::open(CA_CERT_PATH)?;
    let mut ca_cert_reader = BufReader::new(ca_cert_file);
    let ca_certs = rustls_pemfile::certs(&mut ca_cert_reader)?;
    if ca_certs.len() < 1 {
        let msg = "Root server certificate not found";
        println!("{}", msg);
        return Ok(())
    }
    let ca_cert = &ca_certs[0];
    let ca_certificate = rustls::Certificate(ca_cert.to_vec());
    let mut ca_cert_store = rustls::RootCertStore::empty();
    ca_cert_store.add(&ca_certificate).unwrap();

    let cert_file = File::open(CERT_PATH)?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs = rustls_pemfile::certs(&mut cert_reader)?;
    if certs.len() < 1 {
        let msg = "Client certificate not found";
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
                              .with_root_certificates(ca_cert_store)
                              .with_single_cert(certificate, key)
                              .unwrap());
    let tcp_stream = TcpStream::connect(addr);
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
