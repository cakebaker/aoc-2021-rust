use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let depths = read_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&depths));
    println!("Result of puzzle 2: {}", part_2(&depths));

    Ok(())
}

fn part_1(depths: &Vec<i32>) -> usize {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn part_2(depths: &Vec<i32>) -> usize {
    part_1(
        &depths
            .windows(3)
            .map(|triplet| triplet[0] + triplet[1] + triplet[2])
            .collect(),
    )
}

fn read_file(filename: &str) -> Vec<i32> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
