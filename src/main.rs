use std::env;

mod client;
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return println!("Error: Specify a command");
    }

    let server_location = "127.0.0.1:5888".to_string();

    match args[1].as_str() {
        "server" => {
            server::start(server_location);
        }
        "start" => {
            if args.len() < 3 {
                return println!("Error: Specify the ID of what you are tracking");
            }
            client::start_session(args[2].clone(), server_location);
        }
        "stop" => {
            client::stop_session(server_location);
        }
        "status" => {
            // if args.len() < 3 {
            //     return println!("Error: Specify the session ID for more information");
            // }
            client::status(server_location);
        }
        cmd => println!("Error: Unknown command '{}'", cmd),
    }
}
