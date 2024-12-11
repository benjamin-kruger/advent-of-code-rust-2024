pub fn solve() {
    let input = std::fs::read_to_string("src/day01/input.txt").expect("Failed to read input file");
    let lines = input.lines();

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace();
        if let Some(first_number) = numbers.next().map(|x| x.parse::<i32>().unwrap()) {
            col1.push(first_number);
        }
        if let Some(second_number) = numbers.next().map(|x| x.parse::<i32>().unwrap()) {
            col2.push(second_number);
        }
    }

    col1.sort();
    col2.sort();

    let distances = col1.iter().zip(col2.iter()).map(|(a, b)| (a - b).abs());
    let distance: i32 = distances.sum();

    println!("The total distance is: {}", distance);
}
