use std::collections::HashSet;

use tracing::{debug, event, info, trace, Level};

#[derive(Debug)]
pub struct AreaMap {
    map: Vec<Vec<Item>>,
}
impl AreaMap {
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
            None => unreachable!(),
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

pub fn process(mut input: AreaMap) -> usize {
    let mut traversed_points = vec![];
    loop {
        let (location, dir) = input.find_guard();
        let location = location.unwrap();
        let dir = dir.unwrap();

        info!(location = ?location, dir = ?dir, "Location");

        input.map[location.y][location.x] = Item::Vacant;
        info!(next_point = ?input.find_occupied(location, dir), "Next Point: ");
        if let Some(next_point) = input.find_occupied(location, dir) {
            let new_guard_location = match dir {
                Direction::Up => Point {
                    x: location.x,
                    y: next_point.y + 1,
                },
                Direction::Down => Point {
                    x: location.x,
                    y: next_point.y - 1,
                },
                Direction::Left => Point {
                    x: next_point.x + 1,
                    y: location.y,
                },
                Direction::Right => Point {
                    x: next_point.x - 1,
                    y: location.y,
                },
            };

            info!(new_guard_location = ?new_guard_location, "New Guard Location:");
            input.map[new_guard_location.y][new_guard_location.x] = Item::Guard(dir.right_turn());
            info!("{:?} -> {:?}", dir, dir.right_turn());
            match dir {
                Direction::Up => {
                    for y in next_point.y..location.y {
                        traversed_points.push(Point { x: location.x, y });
                    }
                }
                Direction::Down => {
                    for y in location.y..next_point.y {
                        traversed_points.push(Point { x: location.x, y });
                    }
                }
                Direction::Left => {
                    for x in next_point.x..location.x {
                        traversed_points.push(Point { x, y: location.y });
                    }
                }
                Direction::Right => {
                    for x in location.x..next_point.x {
                        traversed_points.push(Point { x, y: location.y });
                    }
                }
            }
        } else {
            match dir {
                Direction::Up => {
                    for y in 0..location.y {
                        traversed_points.push(Point { x: location.x, y });
                    }
                }
                Direction::Down => {
                    for y in location.y..input.map.len() {
                        traversed_points.push(Point { x: location.x, y });
                    }
                }
                Direction::Left => {
                    for x in 0..location.x {
                        traversed_points.push(Point { x, y: location.y });
                    }
                }
                Direction::Right => {
                    for x in location.x..input.map[0].len() {
                        traversed_points.push(Point { x, y: location.y });
                    }
                }
            }
            break;
        }
    }

    let unique_points: HashSet<_> = traversed_points
        .iter()
        .filter(|p| input.map[p.y][p.x] != Item::Occupied)
        .cloned()
        .collect();
    unique_points.len()
}
