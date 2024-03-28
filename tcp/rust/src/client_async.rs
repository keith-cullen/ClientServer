/****************************
 *    Copyright (c) 2024    *
 *    Keith Cullen          *
 ****************************/

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::str::from_utf8;

const NUM_ITER: u32 = 5;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = TcpStream::connect("localhost:12345").await?;
    println!("Opened connection to {}", stream.peer_addr().unwrap());
    handle_connection(stream).await;
    Ok(())
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];
    for i in 0..NUM_ITER {
        println!("Sending");
        let str = format!("hello{}", i);
        let write_res = stream.write(str.as_bytes()).await;
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
        let flush_res = stream.flush().await;
        if let Err(e) = flush_res {
            println!("{}", e);
            return;
        }
        println!("Receiving");
        let read_res = stream.read(&mut buf).await;
        match read_res {
            Ok(n) => {
                if n ==0 {
                    break;
                }
                println!("Received: {}", from_utf8(&buf[0..n]).unwrap());
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
    println!("Connection closed");
}
