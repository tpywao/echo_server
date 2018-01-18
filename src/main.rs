use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::net::SocketAddr;
use std::thread;

trait Handler: Send + Copy + 'static {
    fn handle(self, &mut TcpStream);
}

fn run_server<H: Handler>(config: Config<H>) {
    let listener = TcpListener::bind(config.host).unwrap();
    for stream in listener.incoming() {
        thread::spawn(move || {
            config.handler.handle(
                &mut stream.unwrap())
        });
    }
}

#[derive(Clone, Copy)]
struct Config<H> {
    host: SocketAddr,
    handler: H,
}
impl<H: Handler> Config<H> {
    fn new(host: &str, handler: H) -> Config<H> {
        Config {
            // 不明点: parseでlocalhost:8080が解析できないこと
            host: host.parse().unwrap(),
            handler,
        }
    }
}

#[derive(Clone, Copy)]
struct Echo;
impl Handler for Echo {
    fn handle(self, stream: &mut TcpStream) {
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
}

fn main() {
    let host = "127.0.0.1:8080";
    let config = Config::new(host, Echo);
    run_server(config);
}
