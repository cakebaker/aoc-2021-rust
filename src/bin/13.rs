use std::collections::HashSet;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let (points, instructions) = get_points_and_instructions_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&points, &instructions));

    Ok(())
}

fn part_1(points: &[Point], instructions: &[Instruction]) -> usize {
    let mut visible_points = HashSet::new();

    match instructions.first().unwrap() {
        Instruction::FoldAlongX(fold_x) => {
            for point in points {
                if point.x < *fold_x {
                    visible_points.insert(point.clone());
                } else {
                    visible_points.insert(Point::new((2 * fold_x) - point.x, point.y));
                }
            }
        }
        Instruction::FoldAlongY(fold_y) => {
            for point in points {
                if point.y < *fold_y {
                    visible_points.insert(point.clone());
                } else {
                    visible_points.insert(Point::new(point.x, (2 * fold_y) - point.y));
                }
            }
        }
    }

    visible_points.len()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        let (x, y) = (x.parse()?, y.parse()?);

        Ok(Point::new(x, y))
    }
}

#[derive(Debug)]
enum Instruction {
    FoldAlongX(usize),
    FoldAlongY(usize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s.split_once('=').unwrap();
        let (axis, value) = (axis, value.parse()?);

        if axis == "fold along x" {
            Ok(Instruction::FoldAlongX(value))
        } else {
            Ok(Instruction::FoldAlongY(value))
        }
    }
}

fn get_points_and_instructions_from_file(filename: &str) -> (Vec<Point>, Vec<Instruction>) {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut iter = lines.split(|line| line.is_empty());
    let lines_with_points = iter.next().unwrap();
    let lines_with_instructions = iter.next().unwrap();

    let points = lines_with_points
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    let instructions = lines_with_instructions
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    (points, instructions)
}
