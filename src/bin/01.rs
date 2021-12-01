use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let depths = read_file(&args[0]);
    let result = depths.windows(2).filter(|pair| pair[0] < pair[1]).count();

    println!("Result of puzzle 1: {}", result);

    Ok(())
}

fn read_file(filename: &str) -> Vec<i32> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
