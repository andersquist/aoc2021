use aoc2021::parse;

use std::{fmt, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

#[derive(Clone, PartialEq)]
struct Diagnostics {
    value: u16,
    width: usize,
}

impl FromStr for Diagnostics {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str_radix(s.trim(), 2)
            .map_err(Into::into)
            .map(|value| Self {
                value,
                width: s.len(),
            })
    }
}

impl fmt::Debug for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#012b}", self.value)
    }
}

enum Rating {
    Oxygen,
    Scrubber,
}

#[derive(Debug)]
struct DiagnosticsReport {
    reports: Vec<Diagnostics>,
    gamma: usize,
    epsilon: usize,
    width: usize,
}

impl DiagnosticsReport {
    fn power_consumption(&self) -> usize {
        self.gamma * self.epsilon
    }

    fn get_rating(&self, rating: Rating) -> Result<usize, Error> {
        let mut possible_values = self.reports.clone();
        for position in (0..self.width).rev() {
            let ones = possible_values
                .iter()
                .filter(|r| r.value & 1 << position != 0)
                .count();

            let zeros = possible_values.len() - ones;

            let value_to_keep = match rating {
                Rating::Oxygen if zeros > ones => 0,
                Rating::Oxygen => 1,
                Rating::Scrubber if zeros <= ones => 0,
                Rating::Scrubber => 1,
            };

            possible_values.retain(|r| r.value & 1 << position == value_to_keep << position);

            if possible_values.is_empty() {
                return Err(Error::NoSolution);
            }
            if possible_values.len() == 1 {
                return Ok(possible_values[0].value as usize);
            }
        }
        Err(Error::NoSolution)
    }
}

impl FromIterator<Diagnostics> for DiagnosticsReport {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Diagnostics>,
    {
        let mut counts = vec![0_usize; 16];
        let reports = iter.into_iter().collect::<Vec<_>>();
        let threshold = reports.len() / 2;
        let mut width = 0;
        for report in reports.iter() {
            for i in 0..16 {
                if report.value & 1 << i != 0 {
                    counts[i] += 1;
                }
            }
            width = width.max(report.width);
        }

        let mut gamma: u16 = 0;
        for pos in 0..counts.len() {
            if counts[pos] > threshold {
                gamma |= 1 << pos;
            }
        }

        let mut epsilon = !gamma;
        for pos in width..16 {
            epsilon &= !(1 << pos);
        }

        Self {
            reports,
            gamma: gamma as usize,
            epsilon: epsilon as usize,
            width,
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let report: DiagnosticsReport = parse(input)?.collect();
    println!("{}", report.power_consumption());
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let report: DiagnosticsReport = parse(input)?.collect();
    let generator_rating = report.get_rating(Rating::Oxygen)?;
    let scrubber_rating = report.get_rating(Rating::Scrubber)?;
    println!("{}", generator_rating * scrubber_rating);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Diagnostics(#[from] ParseIntError),
    #[error("no solution found")]
    NoSolution,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_rating() {
        let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .trim();
        let result: DiagnosticsReport = input.lines().map(|s| s.parse().unwrap()).collect();
        assert_eq!(result.get_rating(Rating::Oxygen).unwrap(), 23);
    }

    #[test]
    fn test_scrubber_rating() {
        let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .trim();
        let result: DiagnosticsReport = input.lines().map(|s| s.parse().unwrap()).collect();
        assert_eq!(result.get_rating(Rating::Scrubber).unwrap(), 10);
    }
}
