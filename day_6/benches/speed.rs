fn main() {
    // Run registered benchmarks.
    divan::main();
}

use day_6::part_1::*;
use day_6::*;

#[divan::bench]
pub fn part_1() {
    let _ = tracing::subscriber::set_default(NoSubscriber::default());
    tracing_subscriber::fmt::init();
    let input = divan::black_box(include_str!("../input.txt"));
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
    let _ans = process(map);
}

use day_6::part_2::process as part_2_process;
use tracing::subscriber::NoSubscriber;

#[divan::bench]
pub fn part_2() {
    let _ = tracing::subscriber::set_default(NoSubscriber::default());
    tracing_subscriber::fmt::init();

    let input = divan::black_box(include_str!("../input.txt"));
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
    let _ans = part_2_process(map);
}
