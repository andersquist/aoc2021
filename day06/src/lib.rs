use aoc2021::input::{parse_newline, Separated};

use std::{collections::HashMap, path::Path, str::FromStr};
use thiserror::Error;

fn calculate_fish(data: &str, days: u32) -> Result<usize, Error> {
    let mut school: HashMap<u8, usize> = HashMap::new();
    <Separated<u8>>::from_str(data.trim())
        .expect("failed to parse comma seperated numbers")
        .into_iter()
        .for_each(|v| *school.entry(v).or_insert(0) += 1);

    for _ in 0..days {
        school = iterate(&school);
    }
    Ok(school.into_iter().map(|(_, v)| v).sum())
}

fn iterate(school: &HashMap<u8, usize>) -> HashMap<u8, usize> {
    let mut new_school: HashMap<u8, usize> = HashMap::new();
    for gen in (0..=8).rev() {
        let count = school.get(&gen).unwrap_or(&0_usize);
        if gen == 0 {
            *new_school.entry(6).or_insert(0) += count;
            *new_school.entry(8).or_insert(0) += count;
        } else {
            *new_school.entry(gen - 1).or_insert(0) += count;
        }
    }
    new_school
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
