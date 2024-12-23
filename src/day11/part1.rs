use std::collections::HashMap;

pub fn blink(input: &str, blinks: u64) -> u64 {
    let mut rocks: HashMap<u64, u64> = HashMap::new();
    for n in input.split_whitespace() {
        *rocks.entry(n.parse().unwrap()).or_insert(0) += 1;
    }
    for _ in 0..blinks {
        let mut new_rocks = HashMap::new();
        for (rock, count) in rocks {
            let name = rock.to_string();
            if rock == 0 {
                *new_rocks.entry(1).or_insert(0) += count;
            } else if name.len() % 2 == 0 {
                let (left, right) = name.split_at(name.len() / 2);
                *new_rocks.entry(left.parse().unwrap()).or_insert(0) += count;
                *new_rocks.entry(right.parse().unwrap()).or_insert(0) += count;
            } else {
                *new_rocks.entry(rock * 2024).or_insert(0) += count;
            }
        }
        rocks = new_rocks;
    }
    rocks.into_values().sum::<u64>()
}

pub fn solve() {
    let input = std::fs::read_to_string("src/day11/input.txt").expect("Failed to read input file");
    println!("Part 1 number of rocks: {}", blink(&input, 25));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_stones() {
        let input =
            std::fs::read_to_string("src/day11/test_input.txt").expect("Failed to read input");
        assert_eq!(blink(&input, 25), 55312);
    }
}
