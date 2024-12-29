pub struct Page {
    pub numbers: Vec<i32>,
}

impl Page {
    pub fn new(numbers: Vec<i32>) -> Self {
        Self { numbers }
    }

    pub fn get_middle(&self) -> i32 {
        self.numbers[self.numbers.len() / 2]
    }

    pub fn is_valid(&self, rules: &[(i32, i32)]) -> bool {
        rules.iter().all(|(x, y)| {
            !self.numbers.contains(x)
                || !self.numbers.contains(y)
                || self.numbers.iter().position(|n| n == x).unwrap()
                    < self.numbers.iter().position(|n| n == y).unwrap()
        })
    }
}

pub fn solve() {
    let input = std::fs::read_to_string("src/day05/input.txt").expect("Failed to read input file");
    let (rules, pages) = parse_input(&input);

    println!("Part 1: {}", sum_valid_middle_nums(&rules, &pages));
}

pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Page>) {
    let mut parts = input.split("\n\n");
    let rules = parse_rules(parts.next().unwrap());
    let pages = parse_pages(parts.next().unwrap());
    (rules, pages)
}

pub fn parse_rules(rules_string: &str) -> Vec<(i32, i32)> {
    rules_string
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let left = parts.next().unwrap().trim().parse().unwrap();
            let right = parts.next().unwrap().trim().parse().unwrap();
            (left, right)
        })
        .collect()
}

pub fn parse_pages(pages_string: &str) -> Vec<Page> {
    pages_string
        .lines()
        .map(|line| {
            let numbers = line.split(',').map(|n| n.trim().parse().unwrap()).collect();
            Page::new(numbers)
        })
        .collect()
}

fn sum_valid_middle_nums(rules: &[(i32, i32)], pages: &[Page]) -> i32 {
    pages
        .iter()
        .filter(|page| page.is_valid(rules))
        .map(|page| page.get_middle())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_valid_middle_pages() {
        let input =
            std::fs::read_to_string("src/day05/test_input.txt").expect("Failed to read input");
        let (rules, pages) = parse_input(&input);
        assert_eq!(sum_valid_middle_nums(&rules, &pages), 143);
    }
}
