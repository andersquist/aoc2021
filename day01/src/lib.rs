use aoc2021::parse;

use std::path::Path;
use thiserror::Error;

pub fn part1(input: &Path) -> Result<(), Error> {
    let inputs: Vec<u64> = parse(input)?.collect();
    let (_, count) = inputs.into_iter().fold((0u64, 0u64), |(prev, count), x| {
        if prev > 0 && prev < x {
            (x, count + 1)
        } else {
            (x, count)
        }
    });

    println!("{}", count);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let inputs: Vec<u64> = parse(input)?.collect();
    let (_, count) = inputs
        .windows(3)
        .into_iter()
        .fold((0u64, 0u64), |(prev, count), w| {
            let sum = w.iter().sum::<u64>();
            if prev > 0 && prev < sum {
                (sum, count + 1)
            } else {
                (sum, count)
            }
        });
    println!("{}", count);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
