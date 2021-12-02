use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let depths = get_depths_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&depths));
    println!("Result of puzzle 2: {}", part_2(&depths));

    Ok(())
}

fn part_1(depths: &[i32]) -> usize {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn part_2(depths: &[i32]) -> usize {
    // "a + b + c < b + c + d" can be simplified to "a < d"
    depths
        .windows(4)
        .filter(|quartet| quartet[0] < quartet[3])
        .count()
}

fn get_depths_from_file(filename: &str) -> Vec<i32> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
