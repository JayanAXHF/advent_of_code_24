use std::str::FromStr;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::*;

#[tracing::instrument]
pub fn process(input: &str) -> u64 {
    let equations = Equation::parse(input);
    let possible_equations: u64 = equations
        .par_iter()
        .map(|eq| if eq.is_possible() { eq.test_val } else { 0_u64 })
        .sum();
    possible_equations
}

pub enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(format!("Invalid operation {}", s)),
        }
    }
}

impl Equation {
    pub fn evaluate_expression<T>(input: Vec<T>) -> u64
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
            }
            i += 2;
        }
        total
    }
}
