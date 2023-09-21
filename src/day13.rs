use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u8},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Packet {
    List(Vec<Packet>),
    Literal(u8),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(l), Self::List(o)) => l.cmp(o),
            (Self::Literal(l), Self::Literal(o)) => l.cmp(o),
            (Self::List(l), Self::Literal(o)) => l.cmp(&vec![Self::Literal(*o)]),
            (Self::Literal(l), Self::List(o)) => vec![Self::Literal(*l)].cmp(o),
        }
    }
}

#[derive(Clone)]
struct PacketPair(Packet, Packet);
pub struct PacketPairList(Vec<PacketPair>);

impl Packet {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((Packet::parse_literal, Packet::parse_list))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Self> {
        map(
            delimited(tag("["), separated_list0(tag(","), Packet::parse), tag("]")),
            Self::List,
        )(input)
    }

    fn parse_literal(input: &str) -> IResult<&str, Self> {
        map(u8, Self::Literal)(input)
    }
}

impl PacketPair {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_pair(Packet::parse, newline, Packet::parse), |p| {
            Self(p.0, p.1)
        })(input)
    }

    fn correct(&self) -> bool {
        self.0 < self.1
    }
}

impl PacketPairList {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(pair(newline, newline), PacketPair::parse),
            Self,
        )(input)
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> PacketPairList {
    PacketPairList::parse(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &PacketPairList) -> usize {
    input
        .0
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.correct())
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &PacketPairList) -> usize {
    let mut packets = input
        .0
        .iter()
        .flat_map(|pair| [pair.0.clone(), pair.1.clone()])
        .collect::<Vec<_>>();

    let first_divider = Packet::List(vec![Packet::List(vec![Packet::Literal(2)])]);
    let second_divider = Packet::List(vec![Packet::List(vec![Packet::Literal(6)])]);

    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    packets.sort();

    (packets.iter().position(|p| *p == first_divider).unwrap() + 1)
        * (packets.iter().position(|p| *p == second_divider).unwrap() + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE_INPUT)), 13);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE_INPUT)), 140);
    }
}
