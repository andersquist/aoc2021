use aoc2021::parse;
use std::path::Path;
use thiserror::Error;

#[derive(parse_display::Display, parse_display::FromStr)]
enum Command {
    #[display("forward {0}")]
    Forward(u32),
    #[display("up {0}")]
    Up(u32),
    #[display("down {0}")]
    Down(u32),
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let commands = parse::<Command>(input)?;
    let (depth, horizontal) =
        commands.fold((0u32, 0u32), |(depth, horizontal), command| match command {
            Command::Forward(amount) => (depth, horizontal + amount),
            Command::Up(amount) => (depth - amount, horizontal),
            Command::Down(amount) => (depth + amount, horizontal),
        });
    println!("{}", depth * horizontal);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let commands = parse::<Command>(input)?;
    let (depth, horizontal, _) = commands.fold(
        (0u32, 0u32, 0u32),
        |(depth, horizontal, aim), command| match command {
            Command::Forward(amount) => (depth + (aim * amount), horizontal + amount, aim),
            Command::Up(amount) => (depth, horizontal, aim - amount),
            Command::Down(amount) => (depth, horizontal, aim + amount),
        },
    );
    println!("{}", depth * horizontal);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
