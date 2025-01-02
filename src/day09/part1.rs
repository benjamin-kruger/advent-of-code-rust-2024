pub fn solve() {
    let input = std::fs::read_to_string("src/day09/input.txt").expect("Failed to read input file");
    println!("Part 1: {}", rearrange_fs(&input));
}

pub fn parse(input: &str) -> (Vec<Option<usize>>, Vec<(usize, usize)>) {
    let mut disk: Vec<Option<usize>> = Vec::new(); // parsed input
    let mut blocks: Vec<(usize, usize)> = Vec::new(); // (position of block, length of block)
    for (i, c) in input.trim().chars().enumerate() {
        let n = c.to_string().parse().unwrap();
        if i % 2 == 0 {
            blocks.push((disk.len(), n));
            for _ in 0..n {
                disk.push(Some(i / 2));
            }
        } else {
            for _ in 0..n {
                disk.push(None);
            }
        }
    }
    (disk, blocks)
}

pub fn checksum(disk: Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, &n)| i * n.unwrap_or(0))
        .sum()
}

fn rearrange_fs(input: &str) -> usize {
    let (mut disk, _) = parse(&input);
    loop {
        let first = disk.iter().position(|&x| x.is_none()).unwrap();
        let last = disk.iter().rposition(|&x| x.is_some()).unwrap();
        if first < last {
            disk.swap(first, last);
        } else {
            break;
        }
    }
    checksum(disk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange_fs() {
        let input =
            std::fs::read_to_string("src/day09/test_input.txt").expect("Failed to read input file");
        assert_eq!(rearrange_fs(&input), 1928);
    }
}
