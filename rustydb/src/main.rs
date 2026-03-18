mod db;

use db::Database;
use std::io::{self, Write};

fn main() {
    let mut db = Database::new();

    println!("Simple DB started. Commands: put <k> <v>, get <k>, exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "put" => {
                if parts.len() != 3 {
                    println!("Usage: put <key> <value>");
                    continue;
                }
                db.put(parts[1].to_string(), parts[2].to_string());
                println!("OK");
            }
            "get" => {
                if parts.len() != 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                match db.get(parts[1]) {
                    Some(val) => println!("{}", val),
                    None => println!("Key not found"),
                }
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}