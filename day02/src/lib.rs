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
    use Command::*;

    let commands = parse::<Command>(input)?;
    let (depth, horizontal) =
        commands.fold((0u32, 0u32), |(depth, horizontal), command| match command {
            Forward(amount) => (depth, horizontal + amount),
            Up(amount) => (depth - amount, horizontal),
            Down(amount) => (depth + amount, horizontal),
        });
    println!("{}", depth * horizontal);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    use Command::*;

    let commands = parse::<Command>(input)?;
    let (depth, horizontal, _) = commands.fold(
        (0u32, 0u32, 0u32),
        |(depth, horizontal, aim), command| match command {
            Forward(amount) => (depth + (aim * amount), horizontal + amount, aim),
            Up(amount) => (depth, horizontal, aim - amount),
            Down(amount) => (depth, horizontal, aim + amount),
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
