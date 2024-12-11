pub fn solve() {
    let input = std::fs::read_to_string("src/day01/input.txt").expect("Failed to read input file");
    let lines = input.lines();

    let mut col1: Vec<usize> = Vec::new();
    let mut col2: Vec<usize> = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace();
        if let Some(first_number) = numbers.next().map(|x| x.parse::<usize>().unwrap()) {
            col1.push(first_number);
        }
        if let Some(second_number) = numbers.next().map(|x| x.parse::<usize>().unwrap()) {
            col2.push(second_number);
        }
    }
    let mut similarity_scores: Vec<usize> = Vec::new();
    for number in col1.iter() {
        similarity_scores.push(col2.iter().filter(|x| x == &number).count() * number);
    }
    let similarity_score = similarity_scores.iter().sum::<usize>();
    println!("The similarity score is: {}", similarity_score)
}
