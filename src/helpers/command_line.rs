use crossterm::{style::{Color,ResetColor, SetForegroundColor},
ExecutableCommand,
}

use std::io::{stdin, stdout}

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout()


    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);
}