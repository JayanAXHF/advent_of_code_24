use day_6::part_1::*;
use day_6::*;

pub fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input.txt");
    //     let input = r"....#.....
    // .........#
    // ..........
    // ..#.......
    // .......#..
    // ..........
    // .#..^.....
    // ........#.
    // #.........
    // ......#...
    // ";
    let map: AreaMap = AreaMap::from(input);
    let ans = process(map);
    println!("{}", ans);
}
