use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::Write;


pub fn transmit_data(host: &str, port: u16) {
    let mut stream = TcpStream::connect(
        format!("{}:{}", host, port)
    ).unwrap();

    let mut my_bytes = [0u8; 100];

    for i in 1..my_bytes.len() as u8 {
        my_bytes[i as usize] = i;
        let bytes_num: usize = stream.write(&my_bytes).unwrap();
        println!("Num of  writte bytes: {}", bytes_num);
        thread::sleep(Duration::from_millis(100));
    }
}
