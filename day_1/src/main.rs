use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in lines {
        let split: Vec<&str> = line.split("   ").collect();
        let left: isize = split[0].parse().unwrap();
        let right: isize = split[1].parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }
    // Day 1 Part 1
    left_list.sort();
    right_list.sort();
    let distances = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| {
            let dist = a - b;
            dist.abs()
        })
        .collect::<Vec<isize>>();
    println!("{:?}", distances.iter().sum::<isize>());
    // Day 1 Part 2
    let mut similarities = HashMap::new();
    for i in right_list {
        if left_list.contains(&i) {
            if similarities.contains_key(&i) {
                similarities.insert(i, similarities.get(&i).unwrap() + 1);
            } else {
                similarities.insert(i, 1);
            }
        }
    }
    let sum = similarities
        .iter()
        .map(|(k, v)| k * v)
        .collect::<Vec<isize>>()
        .iter()
        .sum::<isize>();
    println!("{:?}", sum);
}
