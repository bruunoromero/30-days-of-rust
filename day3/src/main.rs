use std::io::{self, Stdin};

fn read_line(stdin: &Stdin) -> String {
    let mut input = String::new();
    stdin.read_line(&mut input).expect("an input");

    return input.to_owned().trim().to_string();
}

fn main() {
    let stdin = io::stdin();
    let n = read_line(&stdin).parse::<i32>().expect("a valid int");

    if n % 2 == 0 && (n >= 2 && n <= 5 || n > 20) {
        println!("Not Weird");
    } else {
        println!("Weird");
    }
}
