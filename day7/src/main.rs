use utils::{read_int, read_string};

fn main() {
    read_int();

    let reversed = read_string()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{reversed}");
}
