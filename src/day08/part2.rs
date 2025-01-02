use crate::day08::part1::{find_antinodes, Grid};
pub fn solve() {
    let input = std::fs::read_to_string("src/day08/input.txt").expect("Failed to read input file");
    let grid = Grid::parse(&input);
    println!(
        "Part 2 - antinodes with resonant frequency: {}",
        find_antinodes(&grid, true).len()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_antinodes_2() {
        let input =
            std::fs::read_to_string("src/day08/test_input.txt").expect("Failed to read input file");
        let grid = Grid::parse(&input);
        let antinodes = find_antinodes(&grid, true);
        assert_eq!(antinodes.len(), 34);
    }
}
