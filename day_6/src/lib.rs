pub mod part_1;
pub mod part_2;

#[derive(Debug, Clone)]
pub struct AreaMap {
    map: Vec<Vec<Item>>,
}
impl AreaMap {
    #[tracing::instrument]
    pub fn find_guard(&self) -> (Option<Point>, Option<Direction>) {
        let mut location = Point { x: 0, y: 0 };
        match self.map.iter().enumerate().find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find(|(_, &val)| {
                    val == Item::Guard(Direction::Up)
                        || val == Item::Guard(Direction::Down)
                        || val == Item::Guard(Direction::Left)
                        || val == Item::Guard(Direction::Right)
                })
                .map(|(j, _)| (i, j))
        }) {
            Some((i, j)) => {
                location.x = j;
                location.y = i;
            }
            None => return (None, None),
        }

        if let Item::Guard(dir) = self.map[location.y][location.x] {
            (Some(location), Some(dir))
        } else {
            (None, None)
        }
    }

    pub fn find_occupied(&self, point: Point, dir: Direction) -> Option<Point> {
        match dir {
            Direction::Up => {
                if point.y == 0 {
                    return None;
                }
                let mut location = point;
                for y in (0..point.y).rev() {
                    if self.map[y][location.x] == Item::Occupied {
                        location.y = y;
                        return Some(location);
                    }
                }
            }
            Direction::Down => {
                let mut location = point;
                if location.y == self.map.len() - 1 {
                    return None;
                }
                for y in (point.y + 1)..self.map.len() {
                    if self.map[y][location.x] == Item::Occupied {
                        location.y = y;
                        return Some(location);
                    }
                }
            }
            Direction::Left => {
                let mut location = point;
                if location.x == 0 {
                    return None;
                }
                for x in (0..point.x).rev() {
                    if self.map[location.y][x] == Item::Occupied {
                        location.x = x;
                        return Some(location);
                    }
                }
            }
            Direction::Right => {
                // TODO: Optimize this
                let mut location = point;
                if location.x == self.map[0].len() - 1 {
                    return None;
                }
                for x in (point.x + 1)..self.map[0].len() {
                    if self.map[location.y][x] == Item::Occupied {
                        location.x = x;
                        return Some(location);
                    }
                }
            }
        };
        None
    }
    fn to_string(&self, visited: Vec<Point>) -> String {
        let mut s = String::new();
        for (row_i, row) in self.map.iter().enumerate() {
            for (col_i, item) in row.iter().enumerate() {
                match item {
                    Item::Vacant => {
                        if visited.contains(&Point { x: col_i, y: row_i }) {
                            s.push('X');
                        } else {
                            s.push('.');
                        }
                    }
                    Item::Occupied => s.push('#'),
                    Item::Guard(dir) => match dir {
                        Direction::Up => s.push('^'),
                        Direction::Down => s.push('v'),
                        Direction::Left => s.push('<'),
                        Direction::Right => s.push('>'),
                    },
                }
            }
            s.push('\n');
        }
        s
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Item {
    Vacant,
    Occupied,
    Guard(Direction),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn right_turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl From<&char> for Item {
    fn from(s: &char) -> Self {
        match s {
            '.' => Item::Vacant,
            '#' => Item::Occupied,
            '^' => Item::Guard(Direction::Up),
            'v' => Item::Guard(Direction::Down),
            '<' => Item::Guard(Direction::Left),
            '>' => Item::Guard(Direction::Right),
            _ => unreachable!(),
        }
    }
}

impl From<&str> for AreaMap {
    fn from(s: &str) -> Self {
        let mut map = vec![];
        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(Item::from(&c));
            }
            map.push(row);
        }
        Self { map }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

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
