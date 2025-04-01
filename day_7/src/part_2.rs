use std::str::FromStr;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::*;

#[tracing::instrument]
pub fn process(input: &str) -> u64 {
    let equations = Equation::parse(input);
    let possible_equations: u64 = equations
        .par_iter()
        .map(|eq| {
            if eq.is_possible_part_2() {
                eq.test_val
            } else {
                0_u64
            }
        })
        .sum();
    possible_equations
}

pub enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            "|" => Ok(Operation::Concatenate),
            _ => Err(format!("Invalid operation {}", s)),
        }
    }
}

impl Equation {
    pub fn evaluate_expression_part_2<T>(input: Vec<T>) -> u64
    where
        T: AsRef<str>,
    {
        let input: Vec<&str> = input.iter().map(|x| x.as_ref()).collect();
        let mut i = 1;
        let mut total: u64 = input[0].parse().unwrap();
        while i < input.len() {
            let operation = input[i].parse::<Operation>().unwrap();
            let value = input[i + 1]
                .parse::<u64>()
                .map_err(|_| format!("Invalid number: {}", input[i + 1]));
            match operation {
                Operation::Add => {
                    total += value.unwrap();
                }
                Operation::Multiply => {
                    total *= value.unwrap();
                }
                Operation::Concatenate => {
                    let new_total: u64 = format!("{}{}", total, value.unwrap()).parse().unwrap();
                    total = new_total;
                }
            }
            i += 2;
        }
        total
    }
    fn generate_possible_combinations_part_2(length: u64) -> Vec<Vec<char>> {
        let options = ['+', '*', '|'];
        let combinations: Vec<Vec<char>> = (0..length as usize)
            .map(|_| options.iter().cloned())
            .multi_cartesian_product()
            .collect();

        combinations
    }

    pub fn is_possible_part_2(&self) -> bool {
        let combinations =
            Equation::generate_possible_combinations_part_2(self.combinations.len() as u64 - 1_u64);
        for combination in combinations {
            let combination: Vec<String> = combination.iter().map(|x| format!("{}", x)).collect();
            let string_self_combinations: Vec<String> =
                self.combinations.iter().map(|x| format!("{}", x)).collect();

            let combination_chunks = string_self_combinations
                .iter()
                .interleave(&combination)
                .collect::<Vec<&String>>();
            let total = Equation::evaluate_expression_part_2(combination_chunks.clone());
            info!(test_val = self.test_val, expr = ?combination_chunks.into_iter().join(" "), total = ?total);
            if total == self.test_val {
                return true;
            }
        }
        false
    }
}
