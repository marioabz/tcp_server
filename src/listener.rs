use std::net::{ TcpListener, TcpStream };
use std::thread;
use std::time::Duration;
use std::io::Read;

const LOCAL_IP: &str = "127.0.0.1";

pub fn run_server(port: u16) {
    let listener = TcpListener::bind(
        format!("{}:{}", LOCAL_IP, port)
    ).unwrap();

    let mut read_bytes = [0u8; 100];
    let mut newest_read_bytes = [1u8; 100];
    let mut stream: TcpStream;
    for mut stream_result in listener.incoming() {
        
        loop {
            match stream_result {
                Ok(ref mut stream) => println!("{:?}", {
                &stream.read(&mut read_bytes);
                    println!("{:?}", read_bytes);
                }),
                Err(_) => panic!(" PANIC !"),
            }
            thread::sleep(Duration::from_millis(200));
            println!("{}", "pending...");
        }
        println!("{}", "-------------------------------");

    }
}
