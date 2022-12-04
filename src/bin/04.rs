use std::cmp;
use std::str::FromStr;
use std::{cmp::Ordering, num::ParseIntError};

#[derive(PartialEq, Copy, Clone)]
struct Interval {
    lower: u32,
    upper: u32,
}

impl Interval {
    fn intersection(&self, other: &Self) -> Option<Interval> {
        let lower = cmp::max(self.lower, other.lower);
        let upper = cmp::min(self.upper, other.upper);
        if lower > upper {
            None
        } else {
            Some(Interval{lower, upper})
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lower == other.lower && self.upper == other.upper {
            Some(Ordering::Equal)
        } else if self.lower >= other.lower && self.upper <= other.upper {
            Some(Ordering::Less)
        } else if self.lower <= other.lower && self.upper >= other.upper {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl FromStr for Interval {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds: Result<Vec<_>, _> = s.split("-").map(|v| v.parse::<u32>()).collect();
        let bounds = bounds?;
        let lower = bounds[0];
        let upper = bounds[1];
        Ok(Interval { lower, upper })
    }
}

fn parse_intervals(input: &str) -> Result<Vec<(Interval, Interval)>, ParseIntError> {
    let lines = input.lines();
    let mut intervals = Vec::new();
    for line in lines {
        let vals: Result<Vec<Interval>, ParseIntError> =
            line.split(",").map(|s| s.parse::<Interval>()).collect();
        let vals = vals?;
        intervals.push((vals[0], vals[1]));
    }
    Ok(intervals)
}

pub fn part_one(input: &str) -> Option<u32> {
    let intervals = parse_intervals(input).ok();
    let intervals = intervals?;
    let count = intervals
        .iter()
        .map(|i| i.0.partial_cmp(&i.1))
        .filter(|r| r.is_some())
        .count();
    Some(u32::try_from(count).ok()?)
}

pub fn part_two(input: &str) -> Option<u32> {
    let intervals = parse_intervals(input).ok();
    let intervals = intervals?;
    let count = intervals
        .iter()
        .map(|i| i.0.intersection(&i.1))
        .filter(|r| r.is_some())
        .count();
    Some(u32::try_from(count).ok()?)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
