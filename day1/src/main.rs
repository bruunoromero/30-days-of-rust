use utils::read_line;

fn main() {
    let i = 4;
    let d = 4.0;
    let s = "HackerRank ";

    let input_i = read_line().trim().parse::<i32>().expect("a valid int");
    let input_d = read_line().trim().parse::<f64>().expect("a valid float");
    let input_s = read_line();

    println!("{}", i + input_i);
    println!("{:.1}", d + input_d);
    println!("{}{}", s, input_s);
}
