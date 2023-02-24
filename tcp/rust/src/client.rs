/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::time::Duration;

const TIMEOUT: Duration = Duration::new(1, 0);
const NUM_ITER: u32 = 5;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("localhost:12345");
    match stream {
        Ok(stream) => {
            println!("Opened connection to {}", stream.peer_addr().unwrap());
            handle_connection(stream);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.set_read_timeout(Some(TIMEOUT)).unwrap();
    stream.set_write_timeout(Some(TIMEOUT)).unwrap();
    for i in 0..NUM_ITER {
        println!("Sending");
        let str = format!("hello{}", i);
        let write_res = stream.write(str.as_bytes());
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
        let flush_res = stream.flush();
        if let Err(e) = flush_res {
            println!("{}", e);
            return;
        }
        println!("Receiving");
        let read_res = stream.read(&mut buffer);
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
