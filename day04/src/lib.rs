use aoc2021::input::{parse_newline, Separated};

use std::{fmt::Debug, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

struct BingoSubsystem {
    random_numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl FromIterator<String> for BingoSubsystem {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        let mut iter = iter.into_iter();
        if let Some(first_line) = iter.next() {
            let random_numbers = <Separated<u32>>::from_str(first_line.trim())
                .expect("failed to parse comma seperated numbers")
                .into_iter()
                .collect();
            let mut boards: Vec<Board> = Vec::new();

            while let Some(block) = iter.next() {
                let board = Board::from_str(&block.trim()).expect("failed to parse board");
                boards.push(board);
            }

            Self {
                random_numbers,
                boards,
            }
        } else {
            panic!("could not convert input")
        }
    }
}

impl BingoSubsystem {
    fn play(&mut self) -> Result<u32, Error> {
        for number in self.random_numbers.iter() {
            for board in self.boards.iter_mut() {
                board.mark_number(*number);
                if board.check_win() {
                    return Ok(board.unmarked_sum() * (*number));
                }
            }
        }
        Err(Error::NoSolution)
    }

    fn find_final_board(&mut self) -> Result<u32, Error> {
        let mut winning_boards: Vec<(Board, u32)> = Vec::new();
        let mut possible_boards = self.boards.clone();

        for number in self.random_numbers.iter() {
            possible_boards
                .iter_mut()
                .for_each(|b| b.mark_number(*number));

            possible_boards.retain(|b| {
                if b.check_win() {
                    winning_boards.push((b.clone(), *number));
                    return false;
                }
                return true;
            });
        }

        if let Some((board, number)) = winning_boards.last() {
            Ok(board.unmarked_sum() * (*number))
        } else {
            Err(Error::NoSolution)
        }
    }
}

#[derive(Debug, parse_display::Display, Clone)]
#[display("[{rows:?}]")]
struct Board {
    rows: Vec<BoardRow>,
}

impl FromStr for Board {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<BoardRow> = Vec::new();

        for line in s.lines() {
            rows.push(BoardRow::from_str(line.trim())?)
        }
        Ok(Self { rows })
    }
}

impl Board {
    fn mark_number(&mut self, number: u32) {
        for row in self.rows.iter_mut() {
            row.mark_number(number);
        }
    }

    fn check_win(&self) -> bool {
        for col in 0..self.rows[0].numbers.len() {
            if self.rows.iter().all(|row| row.numbers[col].1) {
                return true;
            }
        }
        for row in self.rows.iter() {
            if row.numbers.iter().all(|r| r.1) {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for row in self.rows.iter() {
            sum += row
                .numbers
                .iter()
                .filter(|m| !m.1)
                .map(|m| m.0 as u32)
                .sum::<u32>();
        }
        sum
    }
}

#[derive(Debug, parse_display::Display, Clone)]
#[display("{numbers:?}")]
struct BoardRow {
    numbers: Vec<BoardNumber>,
}

impl BoardRow {
    fn mark_number(&mut self, number: u32) {
        self.numbers
            .iter_mut()
            .filter(|(n, _)| *n == number)
            .for_each(|(_, m)| *m = true)
    }
}

impl FromStr for BoardRow {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers: Vec<BoardNumber> = Vec::new();
        for number in s.split_whitespace() {
            let number = u32::from_str(number)
                .map_err(Into::into)
                .map(|number| (number, false))?;
            numbers.push(number);
        }
        Ok(Self { numbers })
    }
}

type BoardNumber = (u32, bool);

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut bingo_system = BingoSubsystem::from_iter(parse_newline::<String>(input)?);
    println!("{}", bingo_system.play()?);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut bingo_system = BingoSubsystem::from_iter(parse_newline::<String>(input)?);
    println!("{}", bingo_system.find_final_board()?);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
