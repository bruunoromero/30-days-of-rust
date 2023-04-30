use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("an input");

    return input;
}

pub fn read_string() -> String {
    return read_line().trim().to_string();
}

pub fn read_int() -> i32 {
    return read_line().trim().parse().expect("a valid int");
}

pub fn read_float() -> f64 {
    return read_line().trim().parse().expect("a valid float");
}
