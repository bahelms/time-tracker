use std::env;

mod client;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return println!("Error: Specify a command");
    }

    match args[1].as_str() {
        "server" => {
            let address = "127.0.0.1".to_string();
            let port = "5888".to_string();
            server::start(address, port);
        }
        "start" => {
            if args.len() < 3 {
                return println!("Error: Specify the ID of what you are tracking");
            }
            client::start_session(args[2].clone())
        }
        "stop" => {
            println!("Stop - TODO");
        }
        "status" => {
            if args.len() < 3 {
                return println!("Error: Specify the session ID for more information");
            }
            println!("Status - TODO");
        }
        cmd => println!("Error: Unknown command '{}'", cmd),
    }
}
