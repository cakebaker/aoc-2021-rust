use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (command, x) = s.split_once(' ').unwrap();
        let x = x.parse()?;

        Ok(match command {
            "forward" => Command::Forward(x),
            "down" => Command::Down(x),
            "up" => Command::Up(x),
            _ => unreachable!(),
        })
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let commands = get_commands_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&commands));
    println!("Result of puzzle 2: {}", part_2(&commands));

    Ok(())
}

fn part_1(commands: &[Command]) -> isize {
    let mut depth = 0;
    let mut horizontal_position = 0;

    for command in commands {
        match command {
            Command::Up(x) => depth -= x,
            Command::Down(x) => depth += x,
            Command::Forward(x) => horizontal_position += x,
        }
    }

    depth * horizontal_position
}

fn part_2(commands: &[Command]) -> isize {
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;

    for command in commands {
        match command {
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
            Command::Forward(x) => {
                horizontal_position += x;
                depth += aim * x;
            }
        }
    }

    depth * horizontal_position
}

fn get_commands_from_file(filename: &str) -> Vec<Command> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
