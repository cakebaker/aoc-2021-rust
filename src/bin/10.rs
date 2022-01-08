use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let navigation_subsystem = get_navigation_subsystem_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&navigation_subsystem));
    println!("Result of puzzle 2: {}", part_2(&navigation_subsystem));

    Ok(())
}

fn part_1(navigation_subsystem: &[Vec<char>]) -> usize {
    let mut error_scores = Vec::new();

    for line in navigation_subsystem {
        let mut stack = Vec::new();

        for c in line {
            if c.is_open() {
                stack.push(c);
            } else {
                let elem = stack.pop().unwrap();

                if c.is_incorrect_close_for(*elem) {
                    error_scores.push(c.get_error_score());
                    break;
                }
            }
        }
    }

    error_scores.iter().sum()
}

fn part_2(navigation_subsystem: &[Vec<char>]) -> usize {
    let mut scores = Vec::new();

    for line in navigation_subsystem {
        let mut stack = Vec::new();
        let mut is_corrupt = false;

        for c in line {
            if c.is_open() {
                stack.push(c);
            } else {
                let elem = stack.pop().unwrap();

                if c.is_incorrect_close_for(*elem) {
                    is_corrupt = true;
                    break;
                }
            }
        }

        if !is_corrupt {
            let mut score = 0;
            stack.reverse();

            for item in stack {
                score *= 5;
                score += item.get_score();
            }

            scores.push(score);
        }
    }

    scores.sort();

    let middle = scores.len() / 2;

    scores[middle]
}

trait ChunkChar {
    fn get_error_score(&self) -> usize;
    fn get_score(&self) -> usize;
    fn is_incorrect_close_for(&self, open: char) -> bool;
    fn is_open(&self) -> bool;
}

impl ChunkChar for char {
    fn get_error_score(&self) -> usize {
        match self {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn get_score(&self) -> usize {
        match self {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        }
    }

    fn is_incorrect_close_for(&self, open: char) -> bool {
        !matches!(
            (open, self),
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>')
        )
    }

    fn is_open(&self) -> bool {
        matches!(self, '(' | '[' | '{' | '<')
    }
}

fn get_navigation_subsystem_from_file(filename: &str) -> Vec<Vec<char>> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
