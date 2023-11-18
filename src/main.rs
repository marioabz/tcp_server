use std::{ env, str };
pub mod listener;
pub mod streamer;

fn main() {
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
        "tcp-receiver" => listener::run_server(port),
        "tcp-emitter" => streamer::transmit_data(port),
        _ => panic!("Options do not match!"),
    }
}
