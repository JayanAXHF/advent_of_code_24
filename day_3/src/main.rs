use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    let matches = parse_input(input);
    let result = matches.iter().map(|x| {
        let nums = calculate(x);
        let nums = nums
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        nums[0] * nums[1]
    });
    let result = result.sum::<i32>();
    println!("{:?}", result);

    let matches = [vec!["do()".to_string()], parse_input_2(input)].concat();
    let chunks = matches.chunks(2);
    let mut sum = 0;
    for chunk in chunks {
        if chunk[0] == "do()" {
            let matches = parse_input(&chunk[1]);
            let result = matches.iter().map(|x| {
                let nums = calculate(x);
                let nums = nums
                    .split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                nums[0] * nums[1]
            });
            let result = result.sum::<i32>();
            sum += result;
        }
    }
    println!("{:?}", sum);
}

fn parse_input(input: &str) -> Vec<String> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut matches = Vec::new();
    for cap in re.captures_iter(input) {
        let full_match = cap[0].to_string();
        matches.push(full_match);
    }
    matches
}

fn calculate(input: &str) -> String {
    let re = Regex::new(r"\((.*?)\)").unwrap();
    if let Some(captures) = re.captures(input) {
        captures[1].to_string()
    } else {
        String::from("0,0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let matches = parse_input(text);
        assert_eq!(matches.len(), 4);
    }

    #[test]
    fn test_parse_input_2() {
        let text = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let matches = parse_input_2(text);
        println!("{:?}", matches);
        assert_eq!(matches.len(), 5);
    }
}

fn parse_input_2(input: &str) -> Vec<String> {
    let re = Regex::new(r"(do\(\)|don\'t\(\))").unwrap();
    let parts: Vec<&str> = re
        .split(input)
        .zip(re.find_iter(input).map(|m| m.as_str()).chain(Some("")))
        .flat_map(|(part, delim)| [part, delim])
        .filter(|s| !s.is_empty())
        .collect();
    let parts = parts.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    parts
}
