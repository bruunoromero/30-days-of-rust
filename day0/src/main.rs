use std::io;

fn main() {
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).expect("a valid input");
    println!("Hello, World.");
    println!("{msg}");
}
