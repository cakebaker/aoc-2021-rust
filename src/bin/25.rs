use std::collections::HashSet;
use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let map = get_map_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&map));

    Ok(())
}

fn part_1(map: &[Vec<char>]) -> usize {
    let max_y = map.len();
    let max_x = map[0].len();

    let mut east_movers = HashSet::new();
    let mut south_movers = HashSet::new();

    for (y, line) in map.iter().enumerate() {
        for (x, elem) in line.iter().enumerate() {
            match elem {
                '>' => {
                    east_movers.insert((x, y));
                }
                'v' => {
                    south_movers.insert((x, y));
                }
                _ => {}
            }
        }
    }

    let mut steps = 0;

    loop {
        steps += 1;

        let mut moved = false;

        let mut new_east_movers = HashSet::new();
        let mut new_south_movers = HashSet::new();

        for current_position @ (x, y) in &east_movers {
            let next_position = ((x + 1) % max_x, *y);

            if east_movers.contains(&next_position) || south_movers.contains(&next_position) {
                new_east_movers.insert(*current_position);
            } else {
                new_east_movers.insert(next_position);
                moved = true;
            }
        }

        for current_position @ (x, y) in &south_movers {
            let next_position = (*x, (y + 1) % max_y);

            if new_east_movers.contains(&next_position) || south_movers.contains(&next_position) {
                new_south_movers.insert(*current_position);
            } else {
                new_south_movers.insert(next_position);
                moved = true;
            }
        }

        if !moved {
            break;
        }

        east_movers = new_east_movers;
        south_movers = new_south_movers;
    }

    steps
}

fn get_map_from_file(filename: &str) -> Vec<Vec<char>> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
