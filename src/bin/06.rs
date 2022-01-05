use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let age_list = get_age_list_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&age_list));
    println!("Result of puzzle 2: {}", part_2(&age_list));

    Ok(())
}

fn part_1(age_list: &[usize]) -> usize {
    count_fish_after_days(age_list, 80)
}

fn part_2(age_list: &[usize]) -> usize {
    count_fish_after_days(age_list, 256)
}

fn count_fish_after_days(age_list: &[usize], days: usize) -> usize {
    let mut counters: [usize; 9] = [0; 9];

    for age in age_list {
        counters[*age] += 1;
    }

    for _ in 1..=days {
        let zeros = counters[0];

        counters.rotate_left(1);

        counters[8] = zeros;
        counters[6] += zeros;
    }

    counters.iter().sum()
}

fn get_age_list_from_file(filename: &str) -> Vec<usize> {
    let mut file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    // remove newline
    file_content.pop();

    file_content
        .split(',')
        .map(|age| age.parse().unwrap())
        .collect()
}
