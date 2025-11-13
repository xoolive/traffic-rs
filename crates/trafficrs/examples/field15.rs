use std::io::{self, BufRead};
use trafficrs::data::field15::{Field15Element, Field15Parser};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };
        if line.is_empty() {
            continue;
        }
        let elements: Vec<Field15Element> = Field15Parser::parse(&line);
        match serde_json::to_string(&elements) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("JSON serialization error: {}", e),
        }
    }
}
