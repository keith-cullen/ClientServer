/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::new(1, 0);

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:12345")?;
    println!("Listening");
    let mut index: u32 = 0;
    loop {
        let res = listener.accept();
        match res {
            Ok((stream, addr)) => {
                println!("Accepted connection from {}", addr);
                thread::spawn(move || {
                    handle_connection(index, stream)
                });
                index = index + 1;
            }
            Err(e) => {
                println!("Warning: {}", e);
            }
        }
    }
}

fn handle_connection(index: u32, mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.set_read_timeout(Some(TIMEOUT)).unwrap();
    stream.set_write_timeout(Some(TIMEOUT)).unwrap();
    println!("<{}> Connection open", index);
    loop {
        println!("<{}> Receiving", index);
        let num;
        let read_res = stream.read(&mut buf);
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
        let write_res = stream.write(&buf[0..num]);
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
        let flush_res = stream.flush();
        if let Err(e) = flush_res {
            println!("<{}> {}", index, e);
            return;
        }
    }
    println!("<{}> Connection closed", index);
}
