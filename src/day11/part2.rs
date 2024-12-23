pub fn solve() {
    let input = std::fs::read_to_string("src/day11/input.txt").expect("Failed to read input");
    println!(
        "Part 2 number of rocks: {}",
        crate::day11::part1::blink(&input, 75)
    );
}
