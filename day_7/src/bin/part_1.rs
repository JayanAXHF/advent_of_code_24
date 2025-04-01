use day_7::{part_1::process, Equation};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input.txt");
    println!("{}", process(input));
}
