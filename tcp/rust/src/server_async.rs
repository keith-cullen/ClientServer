/****************************
 *    Copyright (c) 2024    *
 *    Keith Cullen          *
 ****************************/

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::str::from_utf8;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:12345").await?;
    println!("Listening");
    let mut index: u32 = 0;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);
        tokio::spawn(async move {
            handle_connection(index, stream).await;
        });
        index = index + 1;
    }
}

async fn handle_connection(index: u32, mut stream: TcpStream) {
    let mut buf = vec![0; 1024];
    println!("<{}> Connection open", index);
    loop {
        println!("<{}> Receiving", index);
        let num;
        let read_res = stream.read(&mut buf).await;
        match read_res {
            Ok(n) => {
                if n == 0 {
                    println!("<{}> EOF", index);
                    break;
                }
                println!("<{}> Received: {}", index, from_utf8(&buf[0..n]).unwrap());
                num = n;
            }
            Err(e) => {
                println!("<{}> {}", index, e);
                return;
            }
        }
        println!("<{}> Sending", index);
        let write_res = stream.write(&buf[0..num]).await;
        match write_res {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                println!("<{}> Sent: {}", index, from_utf8(&buf[0..n]).unwrap());
            }
            Err(e) => {
                println!("<{}> {}", index, e);
                return;
            }
        }
        let flush_res = stream.flush().await;
        if let Err(e) = flush_res {
            println!("<{}> {}", index, e);
            return;
        }
    }
    println!("<{}> Connection closed", index);
}
