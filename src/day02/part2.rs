pub fn solve() {
    let input = std::fs::read_to_string("src/day02/input.txt")
        .expect("Failed to read input file");
    
    let safety_counter = input.lines()
        .filter(|line| is_safe_report(line))
        .count();
    
    println!("There were {} safe reports", safety_counter);
}

fn is_safe_report(line: &str) -> bool {
    let levels: Vec<i32> = line.split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    
    is_sequence_safe(&levels) || 
    (0..levels.len()).any(|skip| {
        let modified_levels: Vec<i32> = levels.iter()
            .enumerate()
            .filter(|&(i, _)| i != skip)
            .map(|(_, &level)| level)
            .collect();
        
        is_sequence_safe(&modified_levels)
    })
}

fn is_sequence_safe(levels: &[i32]) -> bool {
    levels.windows(2).all(|window| 
        (window[0] - window[1]).abs() <= 3
    ) && (
        levels.windows(2).all(|window| window[0] < window[1]) || 
        levels.windows(2).all(|window| window[0] > window[1])
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = std::fs::read_to_string("src/day02/test_input.txt").unwrap();
        let lines = input.lines();
        let safety_counter = lines.filter(|line| is_safe_report(line)).count();
        assert_eq!(safety_counter, 4);
    }
}
