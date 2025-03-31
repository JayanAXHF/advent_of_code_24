use day_4::parse_input;
use day_4::part_2::*;

fn main() {
    let input = include_str!("../../input.txt").trim();
    let grid = parse_input(input);
    println!("{}", part_2(grid));
}
