use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::env;
use std::fs;

type Bits = Vec<char>;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let diagnostic_report = get_diagnostic_report_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&diagnostic_report));
    println!("Result of puzzle 2: {}", part_2(&diagnostic_report));

    Ok(())
}

fn part_1(diagnostic_report: &[Bits]) -> usize {
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

    let gamma_rate = binary_to_decimal(&gamma_rate_bits);
    let epsilon_rate = binary_to_decimal(&epsilon_rate_bits);

    gamma_rate * epsilon_rate
}

fn part_2(diagnostic_report: &[Bits]) -> usize {
    let bit_count = diagnostic_report.first().unwrap().len();
    let mut remaining_numbers = diagnostic_report.to_vec();

    for i in 0..bit_count {
        let (with_zero, with_one) = partition_by_bit_position(&remaining_numbers, i);

        remaining_numbers = match with_zero.len().cmp(&with_one.len()) {
            Ordering::Greater => with_zero,
            Ordering::Less => with_one,
            Ordering::Equal => with_one,
        };

        if remaining_numbers.len() == 1 {
            break;
        }
    }

    let oxygen_rating_bits: String = remaining_numbers.first().unwrap().iter().collect();
    let oxygen_rating = binary_to_decimal(&oxygen_rating_bits);

    let mut remaining_numbers = diagnostic_report.to_vec();

    for i in 0..bit_count {
        let (with_zero, with_one) = partition_by_bit_position(&remaining_numbers, i);

        remaining_numbers = match with_zero.len().cmp(&with_one.len()) {
            Ordering::Greater => with_one,
            Ordering::Less => with_zero,
            Ordering::Equal => with_zero,
        };

        if remaining_numbers.len() == 1 {
            break;
        }
    }

    let co2_rating_bits: String = remaining_numbers.first().unwrap().iter().collect();
    let co2_rating = binary_to_decimal(&co2_rating_bits);

    oxygen_rating * co2_rating
}

fn binary_to_decimal(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn partition_by_bit_position(numbers: &[Bits], position: usize) -> (Vec<Bits>, Vec<Bits>) {
    let mut with_zero_at_position = Vec::new();
    let mut with_one_at_position = Vec::new();

    for bits in numbers {
        if let Some(bit) = bits.get(position) {
            if *bit == '0' {
                with_zero_at_position.push(bits.clone());
            } else {
                with_one_at_position.push(bits.clone());
            }
        }
    }

    (with_zero_at_position, with_one_at_position)
}

fn get_diagnostic_report_from_file(filename: &str) -> Vec<Bits> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
