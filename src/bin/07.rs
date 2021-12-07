use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let positions = get_positions_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&positions));

    Ok(())
}

fn part_1(positions: &[i32]) -> i32 {
    let mut lowest_fuel = i32::MAX;
    let max_pos = *positions.iter().max().unwrap();

    for pos in 0..=max_pos {
        let mut fuel = 0;

        for crab_pos in positions {
            fuel += i32::abs(pos - crab_pos);
        }

        if lowest_fuel > fuel {
            lowest_fuel = fuel;
        }
    }

    lowest_fuel
}

fn get_positions_from_file(filename: &str) -> Vec<i32> {
    let mut file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    // remove newline
    file_content.pop();

    file_content
        .split(',')
        .map(|pos| pos.parse().unwrap())
        .collect()
}
