use crate::*;
use rayon::prelude::*;
use std::collections::HashSet;
use tracing::{info, trace, warn};

pub fn process(mut input: AreaMap) -> usize {
    let mut traversed_points = vec![];
    let temp_input = input.clone();
    trace!("Initial Loop");
    loop {
        println!("\n");
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
    let unique_points: Vec<_> = unique_points.into_iter().collect();

    trace!("Checking all traversed points");

    let solutions: usize = unique_points
        .par_iter()
        .enumerate()
        .map(|(i, point)| {
            let mut guard_stops = vec![];
            info!("{:=^50}", "=");
            info!(progress = ?format!("Point {}/{}", i+1, unique_points.len()), pos = ?point);
            if let (Some(location), _) = temp_input.find_guard() {
                if location == *point {
                    warn!("Found guard, skipping");
                    return 0;
                }
            }

            let mut modified = temp_input.clone();
            modified.map[point.y][point.x] = Item::Occupied;
            loop {
                let (location, dir) = modified.find_guard();
                let location = if let Some(location) = location {
                    location
                } else {
                    break 0;
                };
                let dir = dir.unwrap();

                info!(location = ?location, dir = ?dir, "Location");
                if guard_stops.contains(&(location, dir)) {
                    break 1;
                }
                guard_stops.push((location, dir));

                modified.map[location.y][location.x] = Item::Vacant;
                info!(next_point = ?modified.find_occupied(location, dir), "Next Point: ");
                if let Some(next_point) = modified.find_occupied(location, dir) {
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
                    modified.map[new_guard_location.y][new_guard_location.x] =
                        Item::Guard(dir.right_turn());
                    info!("{:?} -> {:?}", dir, dir.right_turn());
                } else {
                    break 0;
                }
            }
        })
        .sum();

    // for (i, point) in unique_points.iter().enumerate() {
    //     let mut guard_stops = vec![];
    //     info!("{:=^50}", "=");
    //     info!(progress = ?format!("Point {}/{}", i+1, unique_points.len()), pos = ?point);
    //     if let (Some(location), _) = temp_input.find_guard() {
    //         if location == *point {
    //             warn!("Found guard, skipping");
    //             continue;
    //         }
    //     }
    //
    //     let mut modified = temp_input.clone();
    //     modified.map[point.y][point.x] = Item::Occupied;
    //     loop {
    //         let (location, dir) = modified.find_guard();
    //         let location = if let Some(location) = location {
    //             location
    //         } else {
    //             break;
    //         };
    //         let dir = dir.unwrap();
    //
    //         info!(location = ?location, dir = ?dir, "Location");
    //         if guard_stops.contains(&(location, dir)) {
    //             solutions += 1;
    //             break;
    //         }
    //         guard_stops.push((location, dir));
    //
    //         modified.map[location.y][location.x] = Item::Vacant;
    //         info!(next_point = ?modified.find_occupied(location, dir), "Next Point: ");
    //         if let Some(next_point) = modified.find_occupied(location, dir) {
    //             let new_guard_location = match dir {
    //                 Direction::Up => Point {
    //                     x: location.x,
    //                     y: next_point.y + 1,
    //                 },
    //                 Direction::Down => Point {
    //                     x: location.x,
    //                     y: next_point.y - 1,
    //                 },
    //                 Direction::Left => Point {
    //                     x: next_point.x + 1,
    //                     y: location.y,
    //                 },
    //                 Direction::Right => Point {
    //                     x: next_point.x - 1,
    //                     y: location.y,
    //                 },
    //             };
    //
    //             info!(new_guard_location = ?new_guard_location, "New Guard Location:");
    //             modified.map[new_guard_location.y][new_guard_location.x] =
    //                 Item::Guard(dir.right_turn());
    //             info!("{:?} -> {:?}", dir, dir.right_turn());
    //             match dir {
    //                 Direction::Up => {
    //                     for y in next_point.y..location.y {
    //                         traversed_points.push(Point { x: location.x, y });
    //                     }
    //                 }
    //                 Direction::Down => {
    //                     for y in location.y..next_point.y {
    //                         traversed_points.push(Point { x: location.x, y });
    //                     }
    //                 }
    //                 Direction::Left => {
    //                     for x in next_point.x..location.x {
    //                         traversed_points.push(Point { x, y: location.y });
    //                     }
    //                 }
    //                 Direction::Right => {
    //                     for x in location.x..next_point.x {
    //                         traversed_points.push(Point { x, y: location.y });
    //                     }
    //                 }
    //             }
    //         } else {
    //             match dir {
    //                 Direction::Up => {
    //                     for y in 0..location.y {
    //                         traversed_points.push(Point { x: location.x, y });
    //                     }
    //                 }
    //                 Direction::Down => {
    //                     for y in location.y..modified.map.len() {
    //                         traversed_points.push(Point { x: location.x, y });
    //                     }
    //                 }
    //                 Direction::Left => {
    //                     for x in 0..location.x {
    //                         traversed_points.push(Point { x, y: location.y });
    //                     }
    //                 }
    //                 Direction::Right => {
    //                     for x in location.x..modified.map[0].len() {
    //                         traversed_points.push(Point { x, y: location.y });
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //     }
    // }
    solutions
}
