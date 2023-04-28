use std::io::{self, Stdin};

fn read_line(stdin: &Stdin) -> String {
    let mut input = String::new();
    stdin.read_line(&mut input).expect("an input");

    return input;
}

fn main() {
    let i = 4;
    let d = 4.0;
    let s = "HackerRank ";

    let stdin = io::stdin();

    let input_i = read_line(&stdin)
        .trim()
        .parse::<i32>()
        .expect("a valid int");
    let input_d = read_line(&stdin)
        .trim()
        .parse::<f64>()
        .expect("a valid float");
    let input_s = read_line(&stdin);

    println!("{}", i + input_i);
    println!("{:.1}", d + input_d);
    println!("{}{}", s, input_s);
}
