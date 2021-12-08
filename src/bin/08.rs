use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let outputs = get_outputs_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&outputs));

    Ok(())
}

fn part_1(outputs: &[Vec<String>]) -> usize {
    let mut count = 0;
    let wanted_lengths = vec![2, 3, 4, 7];

    for output in outputs {
        for digit in output {
            if wanted_lengths.contains(&digit.len()) {
                count += 1;
            }
        }
    }

    count
}

fn get_outputs_from_file(filename: &str) -> Vec<Vec<String>> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content.lines().map(|line| parse(line)).collect()
}

fn parse(s: &str) -> Vec<String> {
    let (_, output) = s.split_once(" | ").unwrap();
    output.split(' ').map(|s| s.to_string()).collect()
}
