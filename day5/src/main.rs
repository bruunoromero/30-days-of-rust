use utils::read_int;

fn main() {
    let n = read_int();

    for i in 1..=10 {
        let mul = n * i;
        println!("{n} x {i} = {mul}");
    }
}
