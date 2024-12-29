use std::collections::{HashMap, HashSet};

pub type Pos = (i32, i32);

pub fn solve() {
    let input = std::fs::read_to_string("src/day06/input.txt").expect("Failed to read input file");

    let grid = parse_grid(&input);
    let visited_count = walk_grid(&grid, None).expect("No valid path found").len();

    println!("Part 1 - Unique visited positions: {}", visited_count);
}

pub fn walk_grid(grid: &HashMap<Pos, char>, obstacle: Option<Pos>) -> Option<HashSet<Pos>> {
    let start = *grid.iter().find(|(_, &c)| c == '^').unwrap().0;
    let mut pos = start;
    let mut dir = (0, -1);
    let mut visited = HashSet::new();

    while visited.insert((pos, dir)) {
        let next = (pos.0 + dir.1, pos.1 - dir.0);

        match (grid.get(&next), obstacle) {
            (Some(&char), obs) if char == '#' || Some(next) == obs => {
                dir = (dir.1, -dir.0); // Turn right
            }
            (Some(_), _) => pos = next, // Move forward
            (None, _) => return Some(visited.iter().map(|&(p, _)| p).collect()),
        }
    }
    None
}

pub fn parse_grid(input: &str) -> HashMap<Pos, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(move |(j, char)| ((i as i32, j as i32), char))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_visited_positions() {
        let input =
            std::fs::read_to_string("src/day06/test_input.txt").expect("Failed to read input");
        let grid = parse_grid(&input);
        let visited_count = walk_grid(&grid, None).expect("No valid path found").len();
        assert_eq!(visited_count, 41);
    }
}
