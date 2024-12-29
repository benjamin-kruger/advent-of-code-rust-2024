use crate::day06::part1::{parse_grid, walk_grid};

pub fn solve() {
    let input = std::fs::read_to_string("src/day06/input.txt").expect("Failed to read input file");

    let grid = parse_grid(&input);
    let initial_path = walk_grid(&grid, None).expect("No valid initial path found");

    let loop_count = initial_path
        .iter()
        .filter(|&pos| grid[pos] == '.' && walk_grid(&grid, Some(*pos)).is_none())
        .count();

    println!("Part 2 - Possible loop positions: {}", loop_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_guard_loops() {
        let input =
            std::fs::read_to_string("src/day06/test_input.txt").expect("Failed to read input");
        let grid = parse_grid(&input);
        let initial_path = walk_grid(&grid, None).expect("No valid initial path found");

        let loop_count = initial_path
            .iter()
            .filter(|&pos| grid[pos] == '.' && walk_grid(&grid, Some(*pos)).is_none())
            .count();
        assert_eq!(loop_count, 6);
    }
}
