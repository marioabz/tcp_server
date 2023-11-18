use std::net::{ TcpListener, TcpStream };
use std::{ thread, env};
use std::time::Duration;
use std::io::Read;


pub fn run_server(host: &str, port: u16) {
    let listener = TcpListener::bind(
        format!("{}:{}", host, port)
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
