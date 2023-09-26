use std::io::{self, Write};

use rpassword::read_password;

fn main() {
    let mut username = String::new();

    print!("input username: ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut username);
    let username = username.trim();

    print!("input password: ");
    io::stdout().flush().unwrap();
    let password: String = read_password().unwrap();
    let password = password.trim();

    println!("id: {username}");
    println!("password: {password}");
}
