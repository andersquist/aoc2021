use aoc2021::parse;

use std::{collections::HashMap, hash::Hash, path::Path};
use thiserror::Error;

struct Map {
    coords: HashMap<Point, usize>,
}

impl Map {
    fn from_lines(lines: Vec<Line>) -> Self {
        let mut coords: HashMap<Point, usize> = HashMap::new();
        for line in lines.iter() {
            line.get_points()
                .iter()
                .for_each(|p| *coords.entry(p.to_owned()).or_insert(0) += 1);
        }
        Self { coords }
    }

    fn find_overlaps(&self) -> usize {
        let mut frequency = self.coords.clone();
        frequency.retain(|_, v| *v > 1);
        frequency.len()
    }
}

#[inline]
fn get_step(n1: i32, n2: i32) -> i32 {
    use std::cmp::Ordering::*;

    match n1.cmp(&n2) {
        Greater => -1,
        Less => 1,
        Equal => 0,
    }
}

#[derive(parse_display::Display, parse_display::FromStr, PartialEq, Debug)]
#[display("{start} -> {end}")]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let mut x = self.start.x;
        let mut y = self.start.y;
        let x_step = get_step(self.start.x, self.end.x);
        let y_step = get_step(self.start.y, self.end.y);
        loop {
            points.push(Point { x, y });
            x += x_step;
            y += y_step;

            if (y_step < 0 && y < self.end.y)
                || (y_step > 0 && y > self.end.y)
                || (x_step < 0 && x < self.end.x)
                || (x_step > 0 && x > self.end.x)
            {
                break;
            }
        }
        points
    }

    #[inline]
    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    #[inline]
    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }
}

#[derive(parse_display::Display, parse_display::FromStr, PartialEq, Debug, Eq, Hash, Clone)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let lines: Vec<Line> = parse::<Line>(input)?
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect();
    let map = Map::from_lines(lines);
    println!("{}", map.find_overlaps());
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let lines: Vec<Line> = parse::<Line>(input)?.collect();
    let map = Map::from_lines(lines);
    println!("{}", map.find_overlaps());
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod test {
    use aoc2021::input::parse_str;

    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            "0,9 -> 5,9".parse(),
            Ok(Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            })
        );
    }

    #[test]
    fn test_get_lines() {
        let line1 = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 2, y: 9 },
        };
        let line2 = Line {
            start: Point { x: 0, y: 8 },
            end: Point { x: 8, y: 0 },
        };
        assert_eq!(
            line1.get_points(),
            [
                Point { x: 0, y: 9 },
                Point { x: 1, y: 9 },
                Point { x: 2, y: 9 }
            ]
        );
        assert_eq!(
            line2.get_points(),
            [
                Point { x: 0, y: 8 },
                Point { x: 1, y: 7 },
                Point { x: 2, y: 6 },
                Point { x: 3, y: 5 },
                Point { x: 4, y: 4 },
                Point { x: 5, y: 3 },
                Point { x: 6, y: 2 },
                Point { x: 7, y: 1 },
                Point { x: 8, y: 0 },
            ]
        );
    }

    #[test]
    fn test_find_overlaps() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let lines = parse_str::<Line>(input)
            .unwrap()
            .filter(|l| l.is_horizontal() || l.is_vertical())
            .collect();
        let map = Map::from_lines(lines);
        assert_eq!(map.find_overlaps(), 5);
    }

    #[test]
    fn test_find_overlaps_diagonal() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let lines = parse_str::<Line>(input).unwrap().collect();
        let map = Map::from_lines(lines);
        assert_eq!(map.find_overlaps(), 12);
    }
}
