use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub struct Pairing((RangeInclusive<u32>, RangeInclusive<u32>));

pub trait FullyContains {
    fn fully_contains(&self, other: &RangeInclusive<u32>) -> bool;
}

impl FullyContains for RangeInclusive<u32> {
    fn fully_contains(&self, other: &RangeInclusive<u32>) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }
}

pub trait Overlaps {
    fn overlaps(&self, other: &RangeInclusive<u32>) -> bool;
}

impl Overlaps for RangeInclusive<u32> {
    fn overlaps(&self, other: &RangeInclusive<u32>) -> bool {
        (self.start() <= other.start() && self.end() >= other.start())
            || (other.start() <= self.start() && other.end() >= self.start())
    }
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(tuple((u32, tag("-"), u32)), |(first, _, second)| {
        RangeInclusive::new(first, second)
    })(input)
}

impl Pairing {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, mut ranges) = separated_list1(tag(","), parse_range)(input)?;
        assert_eq!(ranges.len(), 2);

        let (second, first) = (ranges.pop().unwrap(), ranges.pop().unwrap());
        Ok((rest, Self((first, second))))
    }

    fn fully_contained(&self) -> bool {
        (self.0).0.fully_contains(&(self.0).1) || (self.0).1.fully_contains(&(self.0).0)
    }

    fn overlap(&self) -> bool {
        (self.0).0.overlaps(&(self.0).1)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pairing> {
    let (_, pairings) = separated_list1(newline, Pairing::parse)(input).unwrap();

    pairings
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pairing]) -> usize {
    input
        .iter()
        .filter(|&pairing| pairing.fully_contained())
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pairing]) -> usize {
    input.iter().filter(|&pairing| pairing.overlap()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "2-4,6-8\n\
		 2-3,4-5\n\
		 5-7,7-9\n\
		 2-8,3-7\n\
		 6-6,4-6\n\
		 2-6,4-8"
            )),
            2
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "2-4,6-8\n\
		 2-3,4-5\n\
		 5-7,7-9\n\
		 2-8,3-7\n\
		 6-6,4-6\n\
		 2-6,4-8"
            )),
            4
        );
    }
}
