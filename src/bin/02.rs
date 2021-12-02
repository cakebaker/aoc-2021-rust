use std::env;
use std::fs;

#[derive(Debug)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let commands = get_commands_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&commands));

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

fn get_commands_from_file(filename: &str) -> Vec<Command> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content.lines().map(|line| parse(line)).collect()
}

fn parse(line: &str) -> Command {
    let (a, b) = line.split_once(' ').unwrap();
    let value = b.parse().unwrap();

    match a {
        "forward" => Command::Forward(value),
        "down" => Command::Down(value),
        _ => Command::Up(value),
    }
}
