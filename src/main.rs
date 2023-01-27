use std::io;
use std::io::prelude::*;
use json;

const LOG_END_PATTERN : &str = "<|>\n";

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let mut stdout = io::stdout();
    let mut data = json::JsonValue::new_object();
    loop {
        match stdin.read_line(&mut buffer) {
            Ok(_read) if buffer.ends_with(LOG_END_PATTERN) => {
                data["message"] = buffer.as_str().trim_end_matches(LOG_END_PATTERN).into();
                stdout.write(data.dump().as_bytes()).unwrap();
                stdout.flush().unwrap();
                buffer.clear();
            }
            Ok(read) if read == 0 => {
                break;
            }
            Ok(_) => {},
            Err(_) => break,
        }
    }
}
