use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
pub struct Puzzle {
    input: SensorBeaconList,
    target_y: i64,
}

#[derive(Debug)]
struct SensorBeaconList(Vec<SensorBeaconPair>);
#[derive(Debug)]
struct SensorBeaconPair(Point, Point);

impl SensorBeaconList {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list0(newline, SensorBeaconPair::parse), Self)(input)
    }

    fn iter(&self) -> impl Iterator<Item = &SensorBeaconPair> {
        self.0.iter()
    }
}

impl SensorBeaconPair {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(
                tag("Sensor at "),
                separated_pair(Point::parse, tag(": closest beacon is at "), Point::parse),
            ),
            |(p1, p2)| Self(p1, p2),
        )(input)
    }

    fn manhattan_distance(&self) -> i64 {
        self.0.manhattan_distance(&self.1)
    }

    fn known_at(&self, y: i64) -> RangeInclusive<i64> {
        let mhd = self.manhattan_distance();
        let delta_y = (self.0.y - y).abs();
        let min = self.0.x - mhd + delta_y;
        let max = self.0.x + mhd - delta_y;
        min..=max
    }

    fn can_exclude(&self, &Point { x, y }: &Point) -> bool {
        let mhd = self.manhattan_distance();
        let delta_y = (self.0.y - y).abs();
        let min = self.0.x - mhd + delta_y;
        let max = self.0.x + mhd - delta_y;
        (min..=max).contains(&x)
    }

    fn perimeter_points(&self) -> impl Iterator<Item = Point> {
        let mhd = self.manhattan_distance();

        // top to right
        let tr = (self.0.x..(self.0.x + (mhd + 1))).zip((self.0.y - (mhd + 1))..self.0.y);

        // right to bottom
        let rb = (self.0.x + 1..=(self.0.x + mhd + 1))
            .rev()
            .zip((self.0.y)..self.0.y + mhd + 1);

        // bottom to left
        let bl = ((self.0.x - (mhd + 1))..=self.0.x)
            .rev()
            .zip((self.0.y + 1..=(self.0.y + mhd + 1)).rev());

        // left to top
        let lt = ((self.0.x - (mhd + 1))..self.0.x).zip(((self.0.y - mhd)..=self.0.y).rev());

        tr.chain(rb.chain(bl.chain(lt)))
            .map(|(x, y)| Point { x, y })
    }
}

impl Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(tag("x="), separated_pair(i64, tag(", y="), i64)),
            |(x, y)| Self { x, y },
        )(input)
    }

    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Puzzle {
    fn solve(&self) -> usize {
        let range = self
            .input
            .iter()
            .map(|sbp| sbp.known_at(self.target_y))
            .reduce(|acc, r| *acc.start().min(r.start())..=*acc.end().max(r.end()))
            .unwrap();

        // remove any known beacons
        let mut beacons = self
            .input
            .iter()
            .filter_map(|SensorBeaconPair(_, Point { x, y })| {
                (*y == self.target_y && range.contains(x)).then_some((x, y))
            })
            .collect::<Vec<_>>();
        beacons.sort();
        beacons.dedup();

        (range.end() - range.start() + 1) as usize - beacons.len()
    }

    fn solve2(&self) -> i64 {
        for sbp in self.input.iter() {
            let mut points = sbp.perimeter_points().collect::<Vec<_>>();

            for other in self.input.iter() {
                if std::ptr::eq(sbp, other) {
                    continue;
                }

                points.retain(|p| !other.can_exclude(p));
            }

            if points.len() == 1 {
                return points[0].x * 4_000_000 + points[0].y;
            }
        }
        unreachable!()
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Puzzle {
    Puzzle {
        input: SensorBeaconList::parse(input).unwrap().1,
        target_y: 2_000_000,
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Puzzle) -> usize {
    input.solve()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Puzzle) -> i64 {
    input.solve2()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn known_at() {
        let sbp = SensorBeaconPair(Point { x: 8, y: 7 }, Point { x: 2, y: 10 });
        assert_eq!(sbp.known_at(10), 2..=14);
    }

    #[test]
    fn perimeter_points() {
        let sbp = SensorBeaconPair(Point { x: 0, y: 0 }, Point { x: 1, y: 0 });
        let mut points = sbp.perimeter_points();

        //     -2 -1 +0 +1 +1
        // -2         * <------ start here clockwise
        // -1      *  x  *
        // +0   *  x  S  x  *
        // +1      *  x  *
        // +2         *
        assert_eq!(points.next().unwrap(), Point { x: 0, y: -2 });
        assert_eq!(points.next().unwrap(), Point { x: 1, y: -1 });
        assert_eq!(points.next().unwrap(), Point { x: 2, y: 0 });
        assert_eq!(points.next().unwrap(), Point { x: 1, y: 1 });
        assert_eq!(points.next().unwrap(), Point { x: 0, y: 2 });
        assert_eq!(points.next().unwrap(), Point { x: -1, y: 1 });
        assert_eq!(points.next().unwrap(), Point { x: -2, y: 0 });
        assert_eq!(points.next().unwrap(), Point { x: -1, y: -1 });
        assert_eq!(points.next(), None);
    }

    #[test]
    fn examples_part1() {
        let mut puzzle = input_generator(EXAMPLE_INPUT);
        puzzle.target_y = 10;
        assert_eq!(solve_part1(&puzzle), 26);
    }

    #[test]
    fn examples_part2() {
        let puzzle = input_generator(EXAMPLE_INPUT);
        assert_eq!(solve_part2(&puzzle), 56000011);
    }
}
