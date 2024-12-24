use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("src/day04/input.txt").expect("Failed to read input file");
    println!("Part 2: {}", count_x_mas(&input));
}

fn count_x_mas(input: &str) -> i32 {
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i as i32, j as i32), ch);
        }
    }

    let mut count = 0;
    for &(i, j) in grid.keys() {
        if grid.get(&(i, j)) != Some(&'A') {
            continue;
        }

        let tl = grid.get(&(i - 1, j - 1));
        let tr = grid.get(&(i - 1, j + 1));
        let bl = grid.get(&(i + 1, j - 1));
        let br = grid.get(&(i + 1, j + 1));

        if let (Some(tl), Some(tr), Some(bl), Some(br)) = (tl, tr, bl, br) {
            let both_mas = *tl == 'M' && *br == 'S' && *tr == 'M' && *bl == 'S';
            let both_sam = *tl == 'S' && *br == 'M' && *tr == 'S' && *bl == 'M';
            let mas_sam = *tl == 'M' && *br == 'S' && *tr == 'S' && *bl == 'M';
            let sam_mas = *tl == 'S' && *br == 'M' && *tr == 'M' && *bl == 'S';

            if both_mas || both_sam || mas_sam || sam_mas {
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
    fn test_count_x_mas() {
        let input =
            std::fs::read_to_string("src/day04/test_input.txt").expect("Failed to read input");
        assert_eq!(count_x_mas(&input), 9);
    }
}
