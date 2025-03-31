pub mod part_1;
pub mod part_2;

pub fn parse_input(input: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines = lines.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let rules = lines.split(|x| x.is_empty()).next().expect("No rules");
    let updates = lines.split(|x| x.is_empty()).nth(1).unwrap();
    let updates = updates
        .iter()
        .map(|x| {
            let update = x.split(",").collect::<Vec<&str>>();
            let update = update
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            update
        })
        .collect::<Vec<Vec<String>>>();
    (rules.to_vec(), updates)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let (rules, updates) = parse_input(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        let ans = part_1::part_1(rules, updates);
        assert_eq!(ans, 143);
    }

    #[test]
    fn test_part_2() {
        let (rules, updates) = parse_input(
            r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        let ans = part_2::part_2(rules, updates);
        assert_eq!(ans, 123);
    }
}
