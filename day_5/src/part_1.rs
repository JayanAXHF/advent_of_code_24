use tracing::trace;

#[tracing::instrument]
pub fn part_1(rules: Vec<String>, updates: Vec<Vec<String>>) -> u32 {
    let mut correct_updates = Vec::new();
    for update in updates {
        let mut correct = true;
        for rule in &rules[..] {
            let rule = rule.split("|").collect::<Vec<&str>>();
            let first = update.iter().position(|x| x == rule[0]);
            let second = update.iter().position(|x| x == rule[1]);
            if let (Some(first), Some(second)) = (first, second) {
                if first > second {
                    correct = false;
                    break;
                }
            }
        }
        if correct {
            correct_updates.push(update);
        }
    }
    dbg!(&correct_updates);
    let mut sum = 0;
    let mut sums = Vec::new();
    for update in &correct_updates[..] {
        let middle = update.len() / 2;
        println!("{:?}, {:?}", update.len(), middle);
        println!("{:?}", update[middle]);
        sum += update[middle].parse::<u32>().unwrap();
        sums.push(update[middle].parse::<u32>().unwrap());
    }
    println!("{:?}", sums);
    println!("{:?}", sums.iter().sum::<u32>());
    sum
}
