use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let (numbers, mut boards) = get_numbers_and_boards_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&numbers, &mut boards));

    Ok(())
}

fn part_1(numbers: &[usize], boards: &mut [Board]) -> usize {
    for number in numbers {
        for board in &mut *boards {
            board.mark(*number);

            if board.is_bingo() {
                return board.sum_of_unmarked_fields() * number;
            }
        }
    }

    unreachable!();
}

fn get_numbers_and_boards_from_file(filename: &str) -> (Vec<usize>, Vec<Board>) {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut iter = lines.split(|line| line.is_empty());

    let numbers = iter
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();

    for board_rows in iter {
        let mut board_numbers = Vec::new();

        for board_row in board_rows {
            let board_row: Vec<usize> = board_row
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            board_numbers.push(board_row);
        }

        boards.push(Board::new(&board_numbers));
    }

    (numbers, boards)
}

#[derive(Debug)]
struct Board {
    marked_fields: HashMap<(usize, usize), bool>,
    field_numbers: HashMap<usize, (usize, usize)>,
    is_bingo: bool,
}

impl Board {
    fn new(numbers: &[Vec<usize>]) -> Self {
        let mut marked_fields = HashMap::new();
        let mut field_numbers = HashMap::new();

        for (y, l) in numbers.iter().enumerate() {
            for (x, value) in l.iter().enumerate() {
                marked_fields.insert((x, y), false);
                field_numbers.insert(*value, (x, y));
            }
        }

        Self {
            marked_fields,
            field_numbers,
            is_bingo: false,
        }
    }

    fn is_bingo(&self) -> bool {
        self.is_bingo
    }

    fn mark(&mut self, number: usize) {
        if let Some(field @ (x, y)) = self.field_numbers.get(&number) {
            self.marked_fields.insert(*field, true);

            if (0..5).all(|i| self.marked_fields.get(&(i, *y)) == Some(&true))
                || (0..5).all(|i| self.marked_fields.get(&(*x, i)) == Some(&true))
            {
                self.is_bingo = true;
            }
        }
    }

    fn sum_of_unmarked_fields(&self) -> usize {
        let mut sum = 0;

        for (key, value) in self.field_numbers.iter() {
            if self.marked_fields.get(value) == Some(&false) {
                sum += key;
            }
        }

        sum
    }
}
