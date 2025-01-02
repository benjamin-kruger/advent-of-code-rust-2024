use crate::day09::part1::{checksum, parse};
pub fn solve() {
    let input = std::fs::read_to_string("src/day09/input.txt").expect("Failed to read input file");
    println!("Part 2: {}", rearrange_fs(&input));
}

fn rearrange_fs(input: &str) -> usize {
    let (mut disk, mut blocks) = parse(&input);
    while let Some((block_pos, block_size)) = blocks.pop() {
        if let Some(free_space_pos) = (0..block_pos)
            .filter(|&free_space_pos| {
                (free_space_pos..(free_space_pos + block_size)).all(|k| disk[k].is_none())
            })
            .next()
        {
            for i in 0..block_size {
                disk.swap(free_space_pos + i, block_pos + i);
            }
        }
    }
    checksum(disk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange_fs_2() {
        let input =
            std::fs::read_to_string("src/day09/test_input.txt").expect("Failed to read input file");
        assert_eq!(rearrange_fs(&input), 2858);
    }
}
