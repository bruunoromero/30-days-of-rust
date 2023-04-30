use utils::read_line;

fn calculate_pct(base: f64, pct: i32) -> f64 {
    return base * (pct as f64 / 100.0);
}

fn main() {
    let meal_const = read_line().parse::<f64>().expect("a valid float");
    let tip_pct = read_line().parse::<i32>().expect("a valid int");
    let tax_pct = read_line().parse::<i32>().expect("a valid int");

    let total =
        meal_const + calculate_pct(meal_const, tip_pct) + calculate_pct(meal_const, tax_pct);

    println!("{}", total.round() as i32);
}
