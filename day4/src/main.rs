use std::io;

use crate::board::Board;

mod board;

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("assets/input.txt")?;
    let mut input_sections = input.split("\n\n");

    let numbers_draw: Vec<u8> = input_sections
        .next()
        .expect("The input should have a number draw!")
        .split(',')
        .flat_map(str::parse::<u8>)
        .collect();

    let boards: Vec<Board> = input_sections
        .map(|board| {
            let board_numbers = board
                .lines()
                .flat_map(|row| row.split(' ').flat_map(str::parse::<u8>))
                .collect::<Vec<u8>>();

            Board::new(board_numbers)
        })
        .collect();

    if let Some(score) = get_winning_board_score(&numbers_draw, &boards) {
        println!("Winning board score: {}", score);
    }
    if let Some(score) = get_losing_board_score(&numbers_draw, boards) {
        println!("Losing board score: {}", score);
    }

    Ok(())
}

// Part one
fn get_winning_board_score(numbers_draw: &[u8], boards: &[Board]) -> Option<u16> {
    let mut selected_numbers: Vec<u8> = Vec::new();

    for number in numbers_draw.iter() {
        selected_numbers.push(*number);

        for board in boards.iter() {
            if board.is_won(&selected_numbers) {
                return Some(board.score(&selected_numbers));
            }
        }
    }

    None
}

// Part two
fn get_losing_board_score(numbers_draw: &[u8], mut boards: Vec<Board>) -> Option<u16> {
    let mut selected_numbers: Vec<u8> = Vec::new();

    let mut winning_board: Option<Board> = None;

    for number in numbers_draw.iter() {
        selected_numbers.push(*number);

        boards.retain(|board| !board.is_won(&selected_numbers));

        if boards.len() == 1 {
            winning_board = boards.last().map(|board| board.to_owned());
        }

        if boards.is_empty() {
            return winning_board.map(|board| board.score(&selected_numbers));
        }
    }

    None
}
