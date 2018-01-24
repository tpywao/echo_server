use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::thread;

trait Handler: Send + Copy + 'static {
    fn handle(&self, &mut TcpStream);
}

impl<F> Handler for F
where
    F: Send + Copy + 'static + Fn(&mut TcpStream)
{
    fn handle(&self, stream: &mut TcpStream) {
        (*self)(stream)
    }
}

fn run_server<H: Handler>(config: Config<H>) {
    let listener = TcpListener::bind(config.host).unwrap();
    let handler = config.handler;
    for stream in listener.incoming() {
        thread::spawn(move || {
            handler.handle(
                &mut stream.unwrap())
        });
    }
}

struct Config<H> {
    host: SocketAddr,
    handler: H,
}
impl<H: Handler> Config<H> {
    fn new(host: &str, handler: H) -> Config<H> {
        Config {
            host: host.parse().unwrap(),
            handler,
        }
    }
}

fn echo(stream: &mut TcpStream) {
    let mut buf = [0; 64];
    loop {
        let len = stream.read(&mut buf).unwrap();
        if len == 0 {
            println!("Client Closed.");
            break;
        } else {
            stream.write(&buf[..len]).unwrap();
        }
    }
}

fn main() {
    let host = "127.0.0.1:8080";
    let config = Config::new(host, echo);
    run_server(config);
}
