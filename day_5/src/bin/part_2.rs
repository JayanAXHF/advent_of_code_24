use day_5::parse_input;

fn main() {
    tracing_subscriber::fmt::init();
    let (rules, updates) = parse_input(include_str!("../../input.txt"));
    let ans = day_5::part_2::part_2(rules, updates);
    println!("{}", ans);
}
