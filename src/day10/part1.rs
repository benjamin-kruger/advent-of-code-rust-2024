use std::collections::{HashMap, HashSet, VecDeque};

pub fn parse_grid(input: String) -> HashMap<(i32, i32), i32> {
    input
        .lines()
        .enumerate()
        .map(|(i, j)| {
            j.chars()
                .enumerate()
                .map(move |(k, c)| ((i as i32, k as i32), c.to_string().parse().unwrap()))
        })
        .flatten()
        .collect()
}

pub fn get_path_ratings(grid: HashMap<(i32, i32), i32>) -> Vec<Vec<usize>> {
    let mut path_scores = Vec::new();
    for (&(x, y), &val) in &grid {
        if val != 0 {
            continue;
        }
        let mut count_paths = HashMap::from([((x, y), 1)]);
        let mut remaining_pos = VecDeque::from([(x, y)]);
        let mut processed_pos = HashSet::new();
        while let Some((x, y)) = remaining_pos.pop_front() {
            if !processed_pos.insert((x, y)) {
                continue;
            }
            for (dx, dy) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let next_pos = (x + dx, y + dy);
                if grid.get(&next_pos) == Some(&(grid[&(x, y)] + 1)) {
                    *count_paths.entry(next_pos).or_insert(0) += count_paths[&(x, y)];
                    remaining_pos.push_back(next_pos);
                }
            }
        }
        path_scores.push(
            count_paths
                .into_iter()
                .filter(|c| grid[&c.0] == 9)
                .map(|c| c.1)
                .collect(),
        );
    }
    path_scores
}

fn sum_scores(paths: Vec<Vec<usize>>) -> usize {
    paths.iter().map(Vec::len).sum::<usize>() // We care about the number of different destinations
                                              // not the number of paths
}

pub fn solve() {
    let input = std::fs::read_to_string("src/day10/input.txt").expect("Failed to read input file");
    let grid = parse_grid(input);
    let total_rating = sum_scores(get_path_ratings(grid));
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
        let total_rating = sum_scores(get_path_ratings(grid));
        assert_eq!(total_rating, 36);
    }
}
