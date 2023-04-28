use std::io::{self, Stdin};

fn read_line(stdin: &Stdin) -> String {
    let mut input = String::new();
    stdin.read_line(&mut input).expect("an input");

    return input.to_owned().trim().to_string();
}

fn calculate_pct(base: f64, pct: i32) -> f64 {
    return base * (pct as f64 / 100.0);
}

fn main() {
    let stdin = io::stdin();
    let meal_const = read_line(&stdin).parse::<f64>().expect("a valid float");
    let tip_pct = read_line(&stdin).parse::<i32>().expect("a valid int");
    let tax_pct = read_line(&stdin).parse::<i32>().expect("a valid int");

    let total =
        meal_const + calculate_pct(meal_const, tip_pct) + calculate_pct(meal_const, tax_pct);

    println!("{}", total.round() as i32);
}
