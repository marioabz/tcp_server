use std::{ env, str };
pub mod listener;
pub mod streamer;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    const HOST: &str = "HOST";
    let valid_modes: [&str; 2] = ["tcp-receiver", "tcp-emitter"];
    let mut tcp_mode: String;
    let args: Vec<String> = env::args().collect();
    let mode: &str = args[1].as_str();
    let port: u16 = args[2].parse::<u16>().unwrap();
    if !valid_modes.contains(&mode) {
        panic!("Please input a valid mode!")
    }
    if port > 5000u16 || port < 4000u16 {
        panic!("Port is out of range!");
    }

    match mode {
        "tcp-receiver" => {
            let host: &str = "0.0.0.0";
            listener::run_server(host, port);
        }
        "tcp-emitter" => {
            let host = env::var(HOST).unwrap();
            streamer::transmit_data(&host, port);
        },
        _ => panic!("Options do not match!"),
    }
}
