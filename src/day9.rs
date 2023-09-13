enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction {}", s),
        }
    }
}

pub struct Move {
    d: Direction,
    n: usize,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Default)]
struct State {
    head: Position,
    tail: Position,
    visited: std::collections::HashSet<Position>,
}

impl State {
    fn do_move(&mut self, m: &Move) {
        (0..m.n).for_each(|_| self.do_one_move(&m.d));
    }

    fn do_one_move(&mut self, d: &Direction) {
        match d {
            Direction::Up => {
                if self.tail.y + 1 == self.head.y {
                    if self.tail.x + 1 == self.head.x {
                        self.tail.x += 1;
                    } else if self.head.x + 1 == self.tail.x {
                        self.tail.x -= 1;
                    }
                }

                self.head.y += 1;
                if self.head.y - self.tail.y > 1 {
                    self.tail.y += 1;
                }
            }
            Direction::Right => {
                if self.tail.x + 1 == self.head.x {
                    if self.tail.y + 1 == self.head.y {
                        self.tail.y += 1;
                    } else if self.head.y + 1 == self.tail.y {
                        self.tail.y -= 1;
                    }
                }

                self.head.x += 1;
                if self.head.x - self.tail.x > 1 {
                    self.tail.x += 1;
                }
            }
            Direction::Down => {
                if self.tail.y - 1 == self.head.y {
                    if self.tail.x + 1 == self.head.x {
                        self.tail.x += 1;
                    } else if self.head.x + 1 == self.tail.x {
                        self.tail.x -= 1;
                    }
                }

                self.head.y -= 1;
                if self.tail.y - self.head.y > 1 {
                    self.tail.y -= 1;
                }
            }
            Direction::Left => {
                if self.tail.x - 1 == self.head.x {
                    if self.tail.y + 1 == self.head.y {
                        self.tail.y += 1;
                    } else if self.head.y + 1 == self.tail.y {
                        self.tail.y -= 1;
                    }
                }

                self.head.x -= 1;
                if self.tail.x - self.head.x > 1 {
                    self.tail.x -= 1;
                }
            }
        };

        self.visited.insert(self.tail);
    }

    fn tail_positions(&self) -> usize {
        self.visited.len()
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ');
            let d = split.next().unwrap().into();
            let n = split.next().unwrap().parse().unwrap();
            Move { d, n }
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Move]) -> usize {
    let mut s = State::default();
    input.iter().for_each(|m| s.do_move(m));
    s.tail_positions()
}

#[aoc(day9, part2)]
pub fn solve_part2(_input: &[Move]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_movement() {
        let mut s = State::default();

        s.do_move(&Move {
            d: Direction::Up,
            n: 2,
        });
        assert_eq!(s.head, Position { x: 0, y: 2 });
        assert_eq!(s.tail, Position { x: 0, y: 1 });

        s.tail.y = 2;

        s.do_move(&Move {
            d: Direction::Right,
            n: 2,
        });
        assert_eq!(s.head, Position { x: 2, y: 2 });
        assert_eq!(s.tail, Position { x: 1, y: 2 });

        s.tail.x = 2;

        s.do_move(&Move {
            d: Direction::Down,
            n: 2,
        });
        assert_eq!(s.head, Position { x: 2, y: 0 });
        assert_eq!(s.tail, Position { x: 2, y: 1 });

        s.tail.y = 0;

        s.do_move(&Move {
            d: Direction::Left,
            n: 2,
        });
        assert_eq!(s.head, Position { x: 0, y: 0 });
        assert_eq!(s.tail, Position { x: 1, y: 0 });
    }

    #[test]
    fn diagonal_movement() {
        // first example given
        let mut s = State {
            head: Position { x: 2, y: 2 },
            tail: Position { x: 1, y: 1 },
            ..Default::default()
        };

        s.do_move(&Move {
            d: Direction::Up,
            n: 1,
        });

        assert_eq!(s.head, Position { x: 2, y: 3 });
        assert_eq!(s.tail, Position { x: 2, y: 2 });

        // second example given
        s.head = Position { x: 2, y: 2 };
        s.tail = Position { x: 1, y: 1 };

        s.do_move(&Move {
            d: Direction::Right,
            n: 1,
        });

        assert_eq!(s.head, Position { x: 3, y: 2 });
        assert_eq!(s.tail, Position { x: 2, y: 2 });
    }

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "R 4\n\
		 U 4\n\
		 L 3\n\
		 D 1\n\
		 R 4\n\
		 D 1\n\
		 L 5\n\
		 R 2"
            )),
            13
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "R 4\n\
		 U 4\n\
		 L 3\n\
		 D 1\n\
		 R 4\n\
		 D 1\n\
		 L 5\n\
		 R 2"
            )),
            36
        );
    }
}
