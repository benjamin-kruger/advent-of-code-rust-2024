use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn offset_by(&self, dx: i32, dy: i32, multiplier: i32) -> Option<Position> {
        let new_x = (self.x as i32).checked_add(dx * multiplier)?;
        let new_y = (self.y as i32).checked_add(dy * multiplier)?;

        if new_x >= 0 && new_y >= 0 {
            Some(Position {
                x: new_x as usize,
                y: new_y as usize,
            })
        } else {
            None
        }
    }
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = data.len();
        let width = data.first().map_or(0, |row| row.len());

        Self {
            width,
            height,
            data,
        }
    }

    pub fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    pub fn get_char_positions(&self) -> HashMap<char, Vec<Position>> {
        let mut positions = HashMap::new();

        for (y, row) in self.data.iter().enumerate() {
            for (x, &symbol) in row.iter().enumerate() {
                if symbol != '.' {
                    positions
                        .entry(symbol)
                        .or_insert_with(Vec::new)
                        .push(Position { x, y });
                }
            }
        }

        positions
    }
}

fn calculate_offset(char1: &Position, char2: &Position) -> (i32, i32) {
    (
        char1.x as i32 - char2.x as i32,
        char1.y as i32 - char2.y as i32,
    )
}

pub fn find_antinodes(grid: &Grid, use_resonance: bool) -> HashSet<Position> {
    let mut antinodes = HashSet::new();

    for positions in grid.get_char_positions().values() {
        if positions.len() <= 1 {
            continue;
        }

        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                let (dx, dy) = calculate_offset(&pos1, &pos2);

                if use_resonance {
                    // Part 2: Check all positions along the vector in both directions
                    let mut multiplier = 0;
                    while let Some(pos) = pos1.offset_by(dx, dy, multiplier) {
                        if !grid.is_valid_position(&pos) {
                            break;
                        }
                        antinodes.insert(pos);
                        multiplier += 1;
                    }

                    let mut multiplier = 0;
                    while let Some(pos) = pos2.offset_by(-dx, -dy, multiplier) {
                        if !grid.is_valid_position(&pos) {
                            break;
                        }
                        antinodes.insert(pos);
                        multiplier += 1;
                    }
                } else {
                    // Part 1: Check only immediate next positions
                    if let Some(pos) = pos1.offset_by(dx, dy, 1) {
                        if grid.is_valid_position(&pos) {
                            antinodes.insert(pos);
                        }
                    }
                    if let Some(pos) = pos2.offset_by(-dx, -dy, 1) {
                        if grid.is_valid_position(&pos) {
                            antinodes.insert(pos);
                        }
                    }
                }
            }
        }
    }

    antinodes
}

pub fn solve() {
    let input = std::fs::read_to_string("src/day08/input.txt").expect("Failed to read input file");
    let grid = Grid::parse(&input);
    println!("Part 1 - Antinodes: {}", find_antinodes(&grid, false).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinodes() {
        let input =
            std::fs::read_to_string("src/day08/test_input.txt").expect("Failed to read input file");
        let grid = Grid::parse(&input);
        assert_eq!(find_antinodes(&grid, false).len(), 14);
    }
}
