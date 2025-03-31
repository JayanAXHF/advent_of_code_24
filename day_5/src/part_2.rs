#[tracing::instrument]
pub fn part_2(rules: Vec<String>, updates: Vec<Vec<String>>) -> u32 {
    let mut incorrect_updates = Vec::new();
    for update in updates {
        let mut incorrect = false;
        for rule in &rules[..] {
            let rule = rule.split("|").collect::<Vec<&str>>();
            let first = update.iter().position(|x| x == rule[0]);
            let second = update.iter().position(|x| x == rule[1]);
            if let (Some(first), Some(second)) = (first, second) {
                if first > second {
                    incorrect = true;
                    break;
                }
            }
        }
        if incorrect {
            incorrect_updates.push(update);
        }
    }
    for update in &mut incorrect_updates[..] {
        for _ in 0..update.len() {
            for rule in &rules[..] {
                let rule = rule.split("|").collect::<Vec<&str>>();
                let first = update.iter().position(|x| x == rule[0]);
                let second = update.iter().position(|x| x == rule[1]);
                if let (Some(first), Some(second)) = (first, second) {
                    if first > second {
                        println!("{:?}", update);
                        let temp = update[first].clone();
                        update[first] = update[second].clone();
                        update[second] = temp;
                        println!("{:?}", update);
                    }
                }
            }
        }
    }
    println!("{:?}", incorrect_updates);

    let mut sum = 0;
    for update in &incorrect_updates[..] {
        let middle = update.len() / 2;
        println!("{:?}, {:?}", update.len(), middle);
        println!("{:?}", update[middle]);
        sum += update[middle].parse::<u32>().unwrap();
    }
    sum
}
