use day_7::part_2::process;

fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input.txt");
    println!("{}", process(input));
}
