use std::io;
use std::io::prelude::*;
use json;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let mut stdout = io::stdout();

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(read) => {
                if read == 0 {
                    break;
                }
                let mut data = json::JsonValue::new_object();
                data["message"] = buffer.as_str().into();
                stdout.write(data.dump().as_bytes()).unwrap();
                stdout.flush().unwrap();
                buffer.clear();
            },
            Err(_) => break,
        }
    }
}
