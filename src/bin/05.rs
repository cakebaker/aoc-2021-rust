use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let line_segments = get_line_segments_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&line_segments));
    println!("Result of puzzle 2: {}", part_2(&line_segments));

    Ok(())
}

fn part_1(line_segments: &[LineSegment]) -> usize {
    let mut counters = HashMap::new();

    for line_segment in line_segments
        .iter()
        .filter(|ls| ls.is_horizontal() || ls.is_vertical())
    {
        let points = line_segment.get_points();

        for point in points {
            let counter = counters.entry(point).or_insert(0);
            *counter += 1;
        }
    }

    counters.values().filter(|&count| *count >= 2).count()
}

fn part_2(line_segments: &[LineSegment]) -> usize {
    let mut counters = HashMap::new();

    for line_segment in line_segments {
        let points = line_segment.get_points();

        for point in points {
            let counter = counters.entry(point).or_insert(0);
            *counter += 1;
        }
    }

    counters.values().filter(|&count| *count >= 2).count()
}

/*fn count_overlapping_points(line_segments: &[LineSegment]) -> usize {
    let mut counters = HashMap::new();

    for line_segment in line_segments {
        let points = line_segment.get_points();

        for point in points {
            let counter = counters.entry(point).or_insert(0);
            *counter += 1;
        }
    }

    counters.values().filter(|&count| *count >= 2).count()
}*/

#[derive(Debug, Eq, Hash, PartialEq)]
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

        Ok(Point::new(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        if self.is_horizontal() {
            for x in cmp::min(self.start.x, self.end.x)..=cmp::max(self.start.x, self.end.x) {
                points.push(Point::new(x, self.start.y));
            }
        } else if self.is_vertical() {
            for y in cmp::min(self.start.y, self.end.y)..=cmp::max(self.start.y, self.end.y) {
                points.push(Point::new(self.start.x, y));
            }
        } else {
            let diff = cmp::max(self.start.x, self.end.x) - cmp::min(self.start.x, self.end.x);
            println!("{}", diff);

            let x_multi: isize = if self.start.x < self.end.x { -1 } else { 1 };
            let y_multi: isize = if self.start.y < self.end.y { -1 } else { 1 };

            for step in 0..=diff {
                //               println!("{}", step);
                points.push(Point::new(
                    self.start.x + (step as isize * x_multi) as usize,
                    self.start.y + (step as isize * y_multi) as usize,
                ));
            }

            //            todo!();
        }

        points
    }
}

impl FromStr for LineSegment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();

        Ok(LineSegment::new(start.parse()?, end.parse()?))
    }
}

fn get_line_segments_from_file(filename: &str) -> Vec<LineSegment> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
