use std::io::{stdout, stdin, Write};

use ai::Ai;
use chat::Chat;

mod chat;
mod ai;

fn main() {
    println!("Welcome to your personal AI chat. Enter a message and the AI will respond...");
    let mut ai = Ai::new(Chat::new(), std::env::var("OPENAI_SK").unwrap(), 30);
    loop {
        let mut input = String::new();
        print!("You: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Failed to read from stdin???"); // Fuck it. Just crash on error

        println!("AI: {}", ai.send_message(input));
    }
}
