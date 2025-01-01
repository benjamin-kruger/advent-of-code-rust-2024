use crate::day07::part1::{parse_input, Problem};
use std::collections::HashSet;

pub fn solve() {
    let input = std::fs::read_to_string("src/day07/input.txt").expect("Failed to read input file");
    let result = sum_valid_totals(&input);
    println!("Part 2: {}", result);
}

fn find_possible_results_with_concat(numbers: &[i64]) -> HashSet<i64> {
    if numbers.is_empty() {
        return HashSet::from([0]);
    }
    if numbers.len() == 1 {
        return HashSet::from([numbers[0]]);
    }

    let mut current_results = HashSet::from([numbers[0]]);

    for &num in &numbers[1..] {
        let mut new_results = HashSet::new();
        for &prev_result in &current_results {
            new_results.insert(prev_result + num);
            new_results.insert(prev_result * num);
            new_results.insert(format!("{}{}", prev_result, num).parse::<i64>().unwrap());
        }
        current_results = new_results;
    }

    current_results
}

fn sum_valid_totals(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .filter(|problem| {
            find_possible_results_with_concat(&problem.numbers).contains(&problem.target)
        })
        .map(|problem| problem.target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_valid_totals_with_concat() {
        let input =
            std::fs::read_to_string("src/day07/test_input.txt").expect("Failed to read input");
        assert_eq!(sum_valid_totals(&input), 11387);
    }
}
