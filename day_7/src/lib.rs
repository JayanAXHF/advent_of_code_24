pub mod part_1;
pub mod part_2;
use itertools::Itertools;
use tracing::{debug, info, trace};

pub struct Equation {
    pub test_val: u64,
    combinations: Vec<u64>,
}

impl<T> From<T> for Equation
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let s: &str = s.as_ref();
        let mut split: Vec<&str> = s.split(':').collect();
        let test_val: u64 = split[0].parse().unwrap();
        let mut combinations: Vec<u64> = split[1]
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        Equation {
            test_val,
            combinations,
        }
    }
}

impl Equation {
    pub fn parse(s: &str) -> Vec<Self> {
        let lines = s.lines().collect::<Vec<&str>>();
        lines.into_iter().map(Equation::from).collect()
    }
    fn generate_possible_combinations(length: u64) -> Vec<Vec<char>> {
        let options = ['+', '*'];
        let combinations: Vec<Vec<char>> = (0..length as usize)
            .map(|_| options.iter().cloned())
            .multi_cartesian_product()
            .collect();

        combinations
    }
    pub fn is_possible(&self) -> bool {
        let combinations =
            Equation::generate_possible_combinations(self.combinations.len() as u64 - 1_u64);
        for combination in combinations {
            let combination: Vec<String> = combination.iter().map(|x| format!("{}", x)).collect();
            let string_self_combinations: Vec<String> =
                self.combinations.iter().map(|x| format!("{}", x)).collect();

            let combination_chunks = string_self_combinations
                .iter()
                .interleave(&combination)
                .collect::<Vec<&String>>();
            let total = Equation::evaluate_expression(combination_chunks.clone());
            info!(test_val = self.test_val, expr = ?combination_chunks.into_iter().join(" "), total = ?total);
            if total == self.test_val {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    use tracing_subscriber::fmt::init;

    fn control_combinations(length: u64) -> Vec<Vec<char>> {
        let chars = ['+', '*'];

        let total = 1 << length; // 2^n possibilities
        let mut results = Vec::new();

        for i in 0..total {
            let mut s = Vec::new();
            for j in (0..length).rev() {
                // Iterate over each bit position
                s.push(chars[(i >> j) & 1]); // Extract character using bit shift
            }
            results.push(s);
        }

        results
    }

    #[test]
    fn test_parse() {
        init();
        let length = 7;
        let combinations = Equation::generate_possible_combinations(length);
        let control_combinations = control_combinations(length);
        assert_eq!(combinations, control_combinations);
        tracing::info!("{} {}", combinations.len(), control_combinations.len());
    }

    #[test]
    fn test_is_possible() {
        let inputs = [
            "190: 10 19 ",
            "3267: 81 40 27 ",
            "83: 17 5 ",
            "156: 15 6 ",
            "7290: 6 8 6 15 ",
            "161011: 16 10 13 ",
            "192: 17 8 14 ",
            "21037: 9 7 18 13 ",
            "292: 11 6 16 20 ",
        ];
        let equations = Equation::parse(&inputs.join("\n"));
        let possible_equations: u32 = equations
            .par_iter()
            .map(|eq| if eq.is_possible() { 1_u32 } else { 0_u32 })
            .sum();
        assert_eq!(possible_equations, 3);
    }
}
