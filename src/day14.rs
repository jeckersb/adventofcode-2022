use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct Line(Point, Point);

#[derive(Debug)]
struct Rock(Vec<Point>);

#[derive(Debug)]
pub struct RockList(Vec<Rock>);

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_pair(u64, tag(","), u64), |(x, y)| Self {
            x: x as usize,
            y: y as usize,
        })(input)
    }
}

impl Rock {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(" -> "), Point::parse), Self)(input)
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        Box::new(
            self.0
                .windows(2)
                .flat_map(|p| Line::from_endpoints(p[0], p[1]).points()),
        )
    }
}

impl RockList {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, Rock::parse), Self)(input)
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        Box::new(self.0.iter().flat_map(Rock::points))
    }
}

impl Line {
    fn from_endpoints(p1: Point, p2: Point) -> Self {
        Self(p1, p2)
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        if self.0.x == self.1.x {
            let x = self.0.x;
            let y_range = if self.0.y < self.1.y {
                self.0.y..=self.1.y
            } else {
                self.1.y..=self.0.y
            };

            Box::new(y_range.map(move |y| Point { x, y }))
        } else {
            let y = self.0.y;
            let x_range = if self.0.x < self.1.x {
                self.0.x..=self.1.x
            } else {
                self.1.x..=self.0.x
            };

            Box::new(x_range.map(move |x| Point { x, y }))
        }
    }
}

#[derive(Debug)]
enum Bottom {
    EndlessVoid,
    Floor,
}

#[derive(Debug)]
struct State {
    objects: HashSet<Point>,
    highest_y: usize,
    bottom: Bottom,
}

impl From<&RockList> for State {
    fn from(rocks: &RockList) -> Self {
        let mut highest_y = usize::MIN;

        let objects = rocks
            .points()
            .inspect(|p| highest_y = highest_y.max(p.y))
            .collect();

        Self {
            objects,
            highest_y,
            bottom: Bottom::EndlessVoid,
        }
    }
}

impl Iterator for State {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bottom {
            Bottom::EndlessVoid => self.next_endless_void(),
            Bottom::Floor => self.next_floor(),
        }
    }
}

impl State {
    fn next_endless_void(&mut self) -> Option<Point> {
        let mut pos = Point { x: 500, y: 0 };

        loop {
            // abyss-bound
            if pos.y == self.highest_y {
                return None;
            }

            // settled
            let next_y = pos.y + 1;
            if self.objects.contains(&Point {
                x: pos.x,
                y: next_y,
            }) && self.objects.contains(&Point {
                x: pos.x - 1,
                y: next_y,
            }) && self.objects.contains(&Point {
                x: pos.x + 1,
                y: next_y,
            }) {
                self.objects.insert(pos);
                return Some(pos);
            };

            // move down?
            if !self.objects.contains(&Point {
                x: pos.x,
                y: next_y,
            }) {
                pos.y = next_y;
                continue;
            }

            // move down+left?
            if !self.objects.contains(&Point {
                x: pos.x - 1,
                y: next_y,
            }) {
                pos.x -= 1;
                pos.y = next_y;
                continue;
            }

            // move down+right?
            if !self.objects.contains(&Point {
                x: pos.x + 1,
                y: next_y,
            }) {
                pos.x += 1;
                pos.y = next_y;
                continue;
            }

            unreachable!()
        }
    }

    fn next_floor(&mut self) -> Option<Point> {
        let mut pos = Point { x: 500, y: 0 };

        // source blocked
        if self.objects.contains(&pos) {
            return None;
        }

        loop {
            // settled on the floor
            if pos.y == self.highest_y + 1 {
                self.objects.insert(pos);
                return Some(pos);
            }

            // settled above the floor
            let next_y = pos.y + 1;
            if self.objects.contains(&Point {
                x: pos.x,
                y: next_y,
            }) && self.objects.contains(&Point {
                x: pos.x - 1,
                y: next_y,
            }) && self.objects.contains(&Point {
                x: pos.x + 1,
                y: next_y,
            }) {
                self.objects.insert(pos);
                return Some(pos);
            };

            // move down?
            if !self.objects.contains(&Point {
                x: pos.x,
                y: next_y,
            }) {
                pos.y = next_y;
                continue;
            }

            // move down+left?
            if !self.objects.contains(&Point {
                x: pos.x - 1,
                y: next_y,
            }) {
                pos.x -= 1;
                pos.y = next_y;
                continue;
            }

            // move down+right?
            if !self.objects.contains(&Point {
                x: pos.x + 1,
                y: next_y,
            }) {
                pos.x += 1;
                pos.y = next_y;
                continue;
            }

            unreachable!()
        }
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> RockList {
    RockList::parse(input).unwrap().1
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &RockList) -> usize {
    let state = State::from(input);
    state.count()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &RockList) -> usize {
    let mut state = State::from(input);
    state.bottom = Bottom::Floor;
    state.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE_INPUT)), 24);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE_INPUT)), 93);
    }
}
