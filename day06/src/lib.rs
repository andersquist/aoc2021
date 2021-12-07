use aoc2021::input::{parse_newline, Separated};

use std::{collections::HashMap, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

fn calculate_fish(data: &str, days: u32) -> Result<usize, Error> {
    let school: HashMap<u8, usize> = <Separated<u8>>::from_str(data.trim())?.into_iter().fold(
        HashMap::new(),
        |mut school, v| {
            *school.entry(v).or_insert(0) += 1;
            school
        },
    );

    Ok((0..days)
        .fold(school, |school, _| iterate(&school))
        .into_iter()
        .map(|(_, v)| v)
        .sum::<usize>())
}

fn iterate(school: &HashMap<u8, usize>) -> HashMap<u8, usize> {
    (0..=8).rev().fold(HashMap::new(), |mut new_school, gen| {
        let count = school.get(&gen).unwrap_or(&0_usize);
        if gen == 0 {
            *new_school.entry(6).or_insert(0) += count;
            *new_school.entry(8).or_insert(0) += count;
        } else {
            *new_school.entry(gen - 1).or_insert(0) += count;
        }
        new_school
    })
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let input = parse_newline::<String>(input)?.nth(0).unwrap();
    let amount = calculate_fish(&input, 80)?;
    println!("{}", amount);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let input = parse_newline::<String>(input)?.nth(0).unwrap();
    let amount = calculate_fish(&input, 256)?;
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
    fn test_calculate_fish() {
        let data = "3,4,3,1,2";
        let result = calculate_fish(data, 80).unwrap();
        assert_eq!(result, 5934)
    }

    #[test]
    fn test_calculate_fish_forever() {
        let data = "3,4,3,1,2";
        let result = calculate_fish(data, 256).unwrap();
        assert_eq!(result, 26984457539)
    }
}
