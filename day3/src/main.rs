use utils::read_line;

fn main() {
    let n = read_line().parse::<i32>().expect("a valid int");

    if n % 2 == 0 && (n >= 2 && n <= 5 || n > 20) {
        println!("Not Weird");
    } else {
        println!("Weird");
    }
}
