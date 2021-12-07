use std::collections::BTreeMap;
use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let diagnostic_report = get_diagnostic_report_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&diagnostic_report));

    Ok(())
}

fn part_1(diagnostic_report: &[Vec<char>]) -> usize {
    let mut counters = BTreeMap::new();

    for bits in diagnostic_report {
        for (i, bit) in bits.iter().enumerate() {
            let (zeros, ones) = counters.entry(i).or_insert((0, 0));

            if *bit == '0' {
                *zeros += 1;
            } else {
                *ones += 1;
            }
        }
    }

    let mut gamma_rate_bits = String::new();
    let mut epsilon_rate_bits = String::new();

    for (zeros, ones) in counters.values() {
        if zeros > ones {
            gamma_rate_bits.push('0');
            epsilon_rate_bits.push('1');
        } else {
            gamma_rate_bits.push('1');
            epsilon_rate_bits.push('0');
        }
    }

    let gamma_rate = usize::from_str_radix(&gamma_rate_bits, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_bits, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn get_diagnostic_report_from_file(filename: &str) -> Vec<Vec<char>> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}