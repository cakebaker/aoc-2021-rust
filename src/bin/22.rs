use std::collections::HashSet;
use std::env;
use std::fs;

type Range = (isize, isize);

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let reboot_steps = get_reboot_steps_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(&reboot_steps));

    Ok(())
}

fn part_1(reboot_steps: &[(bool, Range, Range, Range)]) -> usize {
    let mut turned_on_tubes = HashSet::new();

    for (turn_tubes_on, (x_min, x_max), (y_min, y_max), (z_min, z_max)) in reboot_steps {
        if *x_min >= -50
            && *x_max <= 50
            && *y_min >= -50
            && *y_max <= 50
            && *z_min >= -50
            && *z_max <= 50
        {
            for x in *x_min..=*x_max {
                for y in *y_min..=*y_max {
                    for z in *z_min..=*z_max {
                        if *turn_tubes_on {
                            turned_on_tubes.insert((x, y, z));
                        } else {
                            turned_on_tubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    turned_on_tubes.len()
}

fn get_reboot_steps_from_file(filename: &str) -> Vec<(bool, Range, Range, Range)> {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    file_content.lines().map(parse).collect()
}

fn parse(s: &str) -> (bool, Range, Range, Range) {
    let turn_tubes_on = s.starts_with("on");

    let elements: Vec<&str> = s.split(',').collect();

    let elements: Vec<&str> = elements
        .iter()
        .map(|elem| {
            let (_, range) = elem.split_once('=').unwrap();
            range
        })
        .collect();

    let elements: Vec<Range> = elements
        .iter()
        .map(|elem| {
            let (min, max) = elem.split_once("..").unwrap();
            let min = min.parse().unwrap();
            let max = max.parse().unwrap();

            (min, max)
        })
        .collect();

    (turn_tubes_on, elements[0], elements[1], elements[2])
}
