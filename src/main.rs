use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::net::{SocketAddr};
use std::thread;

fn handle(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 64];
    loop {
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            println!("Client Closed.");
            return Ok(())
        } else {
            stream.write(&buf[..len]).unwrap();
        }
    }
}

struct EchoServer {
    host: SocketAddr,
}
impl EchoServer {
    fn new(host: &str) -> EchoServer {
        let host = host.parse().unwrap();
        EchoServer {
            host: host,
        }
    }

    fn listen(self) {
        let listener = TcpListener::bind(self.host).unwrap();
        for stream in listener.incoming() {
            thread::spawn(move || {
                handle(stream.unwrap())
            });
        }
    }
}

fn main() {
    let host = "127.0.0.1:8080";
    let server = EchoServer::new(host);
    server.listen();
}
