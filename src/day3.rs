use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Compartment(HashMap<char, u32>);

struct Rucksack {
    first: Compartment,
    second: Compartment,
}

struct Group(Vec<Rucksack>);

fn priority_of(ch: char) -> u32 {
    if ch.is_lowercase() {
        ch as u32 - 96
    } else {
        ch as u32 - 38
    }
}

impl Rucksack {
    fn duplicate_priority(&self) -> u32 {
        let first_set = HashSet::<char>::from(&self.first);
        let second_set = HashSet::<char>::from(&self.second);
        let both: Vec<_> = first_set.intersection(&second_set).collect();

        assert_eq!(both.len(), 1);

        priority_of(*both[0])
    }

    fn item_set(&self) -> HashSet<char> {
        self.first
            .0
            .keys()
            .copied()
            .chain(self.second.0.keys().copied())
            .collect()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let (first, second) = s.split_at(s.len() / 2);

        Self {
            first: Compartment::from(first),
            second: Compartment::from(second),
        }
    }
}

impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        let mut map = HashMap::new();

        for ch in s.chars() {
            map.entry(ch).and_modify(|entry| *entry += 1).or_insert(1);
        }

        Self(map)
    }
}

impl From<&Compartment> for HashSet<char> {
    fn from(c: &Compartment) -> Self {
        c.0.keys().copied().collect()
    }
}

impl From<Vec<Rucksack>> for Group {
    fn from(rucksacks: Vec<Rucksack>) -> Self {
        Self(rucksacks)
    }
}

impl Group {
    fn badge_priority(&self) -> u32 {
        priority_of(self.badge())
    }

    fn badge(&self) -> char {
        let set = self
            .0
            .iter()
            .map(|rs| rs.item_set())
            .reduce(|acc, i| acc.intersection(&i).copied().collect())
            .unwrap();

        assert_eq!(set.len(), 1);

        *set.iter().next().unwrap()
    }
}

fn parse1(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from).collect()
}

fn parse2(input: &str) -> Vec<Group> {
    let mut groups = Vec::new();

    for group in &input.lines().chunks(3) {
        let rucksacks = group.map(Rucksack::from).collect::<Vec<_>>();
        groups.push(Group::from(rucksacks));
    }

    groups
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let rucksacks = parse1(input);

    rucksacks.iter().map(|rs| rs.duplicate_priority()).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let groups = parse2(input);

    groups.iter().map(|group| group.badge_priority()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(
                "vJrwpWtwJgWrhcsFMMfFFhFp\n\
		 jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
		 PmmdzqPrVvPwwTWBwg\n\
		 wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
		 ttgJtRGJQctTZtZT\n\
		 CrZsJsPPZsGzwwsLwLmpwMDw\n"
            ),
            157
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(
                "vJrwpWtwJgWrhcsFMMfFFhFp\n\
		 jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
		 PmmdzqPrVvPwwTWBwg\n\
		 wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
		 ttgJtRGJQctTZtZT\n\
		 CrZsJsPPZsGzwwsLwLmpwMDw\n"
            ),
            70
        );
    }
}
