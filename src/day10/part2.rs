use crate::day10::part1::{get_path_ratings, parse_grid};

fn sum_ratings(paths: Vec<Vec<usize>>) -> usize {
    paths.iter().flatten().sum::<usize>() // We care about unique paths not the number of
                                          // destinations
}

pub fn solve() {
    let input = std::fs::read_to_string("src/day10/input.txt").expect("Failed to read input file");
    let grid = parse_grid(input);
    let total_rating = sum_ratings(get_path_ratings(grid));
    println!("Total valid paths: {}", total_rating);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_valid_paths() {
        let input =
            std::fs::read_to_string("src/day10/test_input.txt").expect("Failed to read input file");
        let grid = parse_grid(input);
        let total_rating = sum_ratings(get_path_ratings(grid));
        assert_eq!(total_rating, 36);
    }
}
