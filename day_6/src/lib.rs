pub mod part_1;
pub mod part_2;

#[cfg(test)]
mod tests {
    use self::part_1::*;
    use super::*;

    #[test]
    fn part_1() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        let map: AreaMap = AreaMap::from(input);
        let ans = process(map);
        assert_eq!(ans, 41);
    }
}
