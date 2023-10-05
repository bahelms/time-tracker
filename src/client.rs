use chrono::Utc;

pub fn start_session(session_id: String) {
    let time = Utc::now().to_rfc3339();
}

pub fn stop_tracking(session_id: String) {
    println!("Tracking for \"{}\" stopped", session_id);
}

// let parsed = chrono::DateTime::parse_from_rfc3339("2023-10-05T20:48:07.921875828+00:00");
// match parsed {
//     Ok(expr) => {
//         println!("Tracking {} - {} - parsed {}", id, time, expr);
//     }
//     Err(e) => println!("Error {}", e),
// }
