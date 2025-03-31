pub mod part_1;
pub mod part_2;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use part_1::*;
    use part_2::*;

    #[test]
    fn test_part_1() {
        let input = r"
S..S
.AA.
.MM.
X..X
        ";

        let inputs = [
            "XMAS
....
....
....",
            "SAMX
....
....
....",
            "X...
M...
A...
S...",
            "S...
A...
M...
X...",
            "X...
.M..
..A.
...S",
            "...S
..A.
.M..
X...",
            "...X
..M.
.A..
S...",
            "S...
.A..
..M.
...X",
        ];
        for input in inputs {
            let grid = parse_input(input);
            println!("{}", input);
            assert_eq!(find(grid), 1);
        }
    }

    #[test]
    fn test_part_2() {
        let input = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";
        let input = r"M.M
.A.
S.S
        ";
        let grid = parse_input(input);
        assert_eq!(part_2(grid), 1);
    }
}
