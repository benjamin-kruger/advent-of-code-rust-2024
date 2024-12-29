use crate::day05::part1::{parse_input, Page};

pub fn solve() {
    let input = std::fs::read_to_string("src/day05/input.txt").expect("Failed to read input file");
    let (rules, pages) = parse_input(&input);
    println!("Part 2: {}", sum_fixed_middle_nums(&rules, &pages));
}

fn sum_fixed_middle_nums(rules: &[(i32, i32)], pages: &[Page]) -> i32 {
    pages
        .iter()
        .filter(|page| !page.is_valid(rules))
        .filter_map(|page| {
            let mut numbers = page.numbers.clone();
            let mut made_change = true;

            while made_change {
                made_change = false;
                for (x, y) in rules {
                    if numbers.contains(x)
                        && numbers.contains(y)
                        && numbers.iter().position(|n| n == x).unwrap()
                            > numbers.iter().position(|n| n == y).unwrap()
                    {
                        let x_index = numbers.iter().position(|n| n == x).unwrap();
                        let y_index = numbers.iter().position(|n| n == y).unwrap();
                        numbers.swap(x_index, y_index);
                        made_change = true;
                    }
                }
            }

            let fixed_page = Page::new(numbers);
            if fixed_page.is_valid(rules) {
                Some(fixed_page.get_middle())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_fixed_middle_pages() {
        let input =
            std::fs::read_to_string("src/day05/test_input.txt").expect("Failed to read input");
        let (rules, pages) = parse_input(&input);
        assert_eq!(sum_fixed_middle_nums(&rules, &pages), 123);
    }
}
