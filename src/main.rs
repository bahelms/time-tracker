use clap::Parser;
use client::TimeTrackerCLI;

mod client;
mod server;

fn main() {
    let server_location = "127.0.0.1:5888".to_string();

    match TimeTrackerCLI::parse() {
        TimeTrackerCLI::Server => {
            server::start(server_location);
        }
        TimeTrackerCLI::Start(params) => {
            client::start_session(server_location, params.session_name);
        }
        TimeTrackerCLI::Stop => {
            client::stop_session(server_location);
        }
        TimeTrackerCLI::Status(params) => {
            client::status(server_location, params);
        }
    }
}
