use day_5::parse_input;

fn main() {
    tracing_subscriber::fmt::init();
    let (rules, updates) = parse_input(include_str!("../../input.txt"));
    let ans = day_5::part_1::part_1(rules, updates);
    println!("{}", ans);
}
