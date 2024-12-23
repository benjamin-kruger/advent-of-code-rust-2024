use regex::Regex;

pub fn solve() {
    let input = std::fs::read_to_string("src/day03/input.txt").expect("Failed to read input file");
    println!("part 2 sum: {}", match_mul_a_b(&input));
}

pub fn match_mul_a_b(input: &str) -> i32 {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut match_mul = true;
    mul_pattern
        .captures_iter(input)
        .map(|cap| {
            if cap.get(0).unwrap().as_str() == "do()" {
                match_mul = true;
                return 0;
            }
            if cap.get(0).unwrap().as_str() == "don't()" {
                match_mul = false;
                return 0;
            }

            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();

            if match_mul {
                x * y
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = std::fs::read_to_string("src/day03/test_input.txt").unwrap();
        assert_eq!(161, match_mul_a_b(&input));
    }
}
