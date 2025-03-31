use std::cmp::Ordering::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let reports = lines
        .iter()
        .map(|line| {
            let report = line
                .split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            report
        })
        .collect::<Vec<Vec<i32>>>();
    let mut safe_reports = vec![];
    for report in reports.clone() {
        let mut safe = true;
        let mut prev_level = report[0];
        let change_type = if report[0] < report[1] { 0 } else { 1 };
        for level in report[1..].iter() {
            match change_type {
                0 => {
                    if *level <= prev_level {
                        safe = false;
                        break;
                    }
                }
                1 => {
                    if *level >= prev_level {
                        safe = false;
                        break;
                    }
                }
                _ => {}
            }
            if (level - prev_level).abs() > 3 {
                safe = false;
                break;
            }
            prev_level = *level;
        }
        if safe {
            safe_reports.push(report);
        }
    }
    println!("{}", safe_reports.len());

    let mut safe_reports = vec![];
    // part 2
    for report in reports {
        let mut safe = check_safe(&report);
        if safe {
            safe_reports.push(report.clone());
            continue;
        }
        for i in 0..report.len() {
            let mut temp_report = report.clone();
            temp_report.remove(i);
            let temp_safe = check_safe(&temp_report);
            if temp_safe {
                safe_reports.push(report.clone());
                break;
            }
        }
    }
    println!("{}", safe_reports.len());
}

fn check_safe(report: &Vec<i32>) -> bool {
    let mut safe = true;
    let mut prev_level = report[0];
    let mut bad_levels = 0;
    let start = 1;
    let change_type = match report[1].cmp(&report[0]) {
        Equal => {
            bad_levels += 1;
            match report[2].cmp(&report[0]) {
                Less => Less,
                Greater => Greater,
                Equal => return false,
            }
        }
        x => x,
    };
    for level in report[start..].iter() {
        let temp_prev_level = prev_level;
        prev_level = *level;
        if (level - temp_prev_level).abs() > 3 {
            bad_levels += 1;
            if bad_levels > 1 {
                safe = false;
                break;
            }
        }
        match change_type {
            Less => {
                if *level >= temp_prev_level {
                    bad_levels += 1;
                    if bad_levels > 1 {
                        safe = false;
                        break;
                    }
                }
            }
            Greater => {
                if *level <= temp_prev_level {
                    bad_levels += 1;
                    if bad_levels > 1 {
                        safe = false;
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    safe
}
