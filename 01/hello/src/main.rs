use std::io;
use std::io::Write;

// Trim newline from string
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    let mut name = String::new();
    io::stdout().write(b"What's Your name ?").unwrap();
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut name) {
        Ok(_n) => {
            trim_newline(&mut name);
            println!("Hello, {}!", name);
        }
        Err(error) => println!("error: {}", error),
    }
}
