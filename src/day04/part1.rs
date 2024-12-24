use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("src/day04/input.txt").expect("Failed to read input file");
    println!("Part 1: {}", count_xmas(&input));
}

fn count_xmas(input: &str) -> i32 {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i as i32, j as i32), ch);
        }
    }

    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let target = "XMAS";
    let mut count = 0;

    for &(i, j) in grid.keys() {
        for &(u, v) in &directions {
            let mut valid_word = true;

            for k in 0..target.len() {
                let next_pos = (i + u * k as i32, j + v * k as i32);
                if !grid.contains_key(&next_pos)
                    || grid.get(&next_pos).unwrap() != &target.chars().nth(k).unwrap()
                {
                    valid_word = false;
                    break;
                }
            }

            if valid_word {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() {
        let input =
            std::fs::read_to_string("src/day04/test_input.txt").expect("Failed to read input");
        assert_eq!(count_xmas(&input), 18);
    }
}
