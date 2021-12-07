use aoc2021::input::{parse_newline, Separated};

use std::{num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

fn calculate_fuel(data: &str) -> Result<i32, Error> {
    calculate_lowest_fuel(data, |sub, pos| (sub - pos).abs())
}

fn calculate_fuel_part2(data: &str) -> Result<i32, Error> {
    calculate_lowest_fuel(data, |sub, pos| {
        let d = (sub - pos).abs();
        d * (d + 1) / 2
    })
}

fn calculate_lowest_fuel(data: &str, fuel_calc: impl Fn(i32, i32) -> i32) -> Result<i32, Error> {
    let crabs: Vec<i32> = <Separated<i32>>::from_str(data.trim())?
        .into_iter()
        .collect();
    let min = crabs.iter().min().expect("no min");
    let max = crabs.iter().max().expect("no max");
    Ok((*min..=*max)
        .map(|pos| {
            crabs
                .iter()
                .copied()
                .map(|c| fuel_calc(c, pos))
                .sum::<i32>()
        })
        .min()
        .expect("no solution found"))
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = parse_newline::<String>(input)?.nth(0).unwrap();
    let amount = calculate_fuel(&input)?;
    println!("{}", amount);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let input = parse_newline::<String>(input)?.nth(0).unwrap();
    let amount = calculate_fuel_part2(&input)?;
    println!("{}", amount);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseError(#[from] ParseIntError),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        let data = "16,1,2,0,4,2,7,1,2,14";
        let result = calculate_fuel(data).unwrap();
        assert_eq!(result, 37)
    }

    #[test]
    fn test_calculate_fuel_part2() {
        let data = "16,1,2,0,4,2,7,1,2,14";
        let result = calculate_fuel_part2(data).unwrap();
        assert_eq!(result, 168)
    }
}
