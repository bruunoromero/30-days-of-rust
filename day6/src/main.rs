use utils::{read_int, read_string};

fn chars_to_string(chars: Vec<char>) -> String {
    let mut str = String::new();

    for char in chars {
        str.push(char);
    }

    return str;
}

fn execute(str: String) {
    let chars = str.chars();
    let even_chars: Vec<char> = chars.clone().step_by(2).collect();
    let odd_chars: Vec<char> = chars.skip(1).step_by(2).collect();

    println!(
        "{} {}",
        chars_to_string(even_chars),
        chars_to_string(odd_chars)
    );
}

fn main() {
    let exec_times = read_int();
    let strs: Vec<String> = (0..exec_times).into_iter().map(|_| read_string()).collect();

    for str in strs {
        execute(str);
    }
}
