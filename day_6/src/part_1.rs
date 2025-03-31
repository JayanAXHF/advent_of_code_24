use crate::*;
use std::collections::HashSet;

use tracing::{debug, event, info, trace, Level};

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
