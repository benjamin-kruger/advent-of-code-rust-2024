mod day01;
mod day02;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }
    let day = args[1].parse::<u8>().unwrap_or_else(|_| {
        exit_with_error("Error: Day must be between 1 and 25, inclusive.");
    });
    if !(1..25).contains(&day) {
        exit_with_error("Error: Day must be between 1 and 25, inclusive.");
    }
    match day {
        1 => day01::run(),
        2 => day02::run(),
        _ => println!("Day {} is not yet implemented", day),
    }
}

fn exit_with_error(message: &str) -> ! {
    eprintln!("{}", message);
    std::process::exit(1);
}
