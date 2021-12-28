use std::env;
use std::fs;

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return Err("No file specified");
    }

    let (player_1, player_2) = get_starting_positions_from_file(&args[0]);

    println!("Result of puzzle 1: {}", part_1(player_1, player_2));

    Ok(())
}

fn part_1(player_1: usize, player_2: usize) -> usize {
    const DICE_ROLLS_PER_ROUND: usize = 3;

    let mut score_player_1 = 0;
    let mut score_player_2 = 0;
    let mut dice = 0;
    let mut position_player_1 = player_1;
    let mut position_player_2 = player_2;

    let mut is_player_1 = true;
    let mut total_dice_rolls = 0;

    while score_player_1 < 1000 && score_player_2 < 1000 {
        let mut dice_sum = 0;

        for _ in 0..DICE_ROLLS_PER_ROUND {
            dice = next_dice(dice);
            dice_sum += dice;
        }

        total_dice_rolls += DICE_ROLLS_PER_ROUND;

        if is_player_1 {
            position_player_1 = calculate_new_position(position_player_1, dice_sum);
            score_player_1 += position_player_1;
        } else {
            position_player_2 = calculate_new_position(position_player_2, dice_sum);
            score_player_2 += position_player_2;
        }

        is_player_1 = !is_player_1;
    }

    usize::min(score_player_1, score_player_2) * total_dice_rolls
}

fn calculate_new_position(current_position: usize, moves: usize) -> usize {
    let mut position = current_position + moves;
    position %= 10;

    if position == 0 {
        10
    } else {
        position
    }
}

fn next_dice(current_dice: usize) -> usize {
    const MAX_DICE: usize = 100;

    if current_dice == MAX_DICE {
        1
    } else {
        current_dice + 1
    }
}

fn get_starting_positions_from_file(filename: &str) -> (usize, usize) {
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let positions: Vec<usize> = file_content
        .lines()
        .map(|line| {
            let (_, position) = line.split_once(": ").unwrap();
            position.parse().unwrap()
        })
        .collect();

    (positions[0], positions[1])
}
