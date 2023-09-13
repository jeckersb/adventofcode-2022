#[derive(Debug)]
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

#[derive(Debug)]
pub struct Move {
    d: Direction,
    n: usize,
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

struct State<const N: usize> {
    nodes: [Position; N],
    visited: std::collections::HashSet<Position>,
}

impl<const N: usize> State<N> {
    fn new() -> Self {
        Self {
            nodes: [Position::default(); N],
            visited: Default::default(),
        }
    }

    fn do_move(&mut self, m: &Move) {
        for _ in 0..m.n {
            self.move_head(&m.d);
            self.move_followers();
            self.visited.insert(self.nodes[N - 1]);
        }
    }

    fn move_head(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.nodes[0].y += 1,
            Direction::Down => self.nodes[0].y -= 1,
            Direction::Right => self.nodes[0].x += 1,
            Direction::Left => self.nodes[0].x -= 1,
        }
    }

    fn move_followers(&mut self) {
        for head in 0..N - 1 {
            self.move_tail(head, head + 1)
        }
    }

    fn move_tail(&mut self, head_idx: usize, tail_idx: usize) {
        let head = self.nodes[head_idx];
        let tail = self.nodes[tail_idx];

        let d_x = head.x - tail.x;
        let d_y = head.y - tail.y;

        match (d_x, d_y) {
            // adjacent, no move
            (0, 0)
            | (1, 0)
            | (0, 1)
            | (0, -1)
            | (-1, 0)
            | (1, 1)
            | (-1, -1)
            | (1, -1)
            | (-1, 1) => (),

            // up
            (0, 2) => self.nodes[tail_idx].y += 1,

            // down
            (0, -2) => self.nodes[tail_idx].y -= 1,

            // left
            (-2, 0) => self.nodes[tail_idx].x -= 1,

            // right
            (2, 0) => self.nodes[tail_idx].x += 1,

            // up-right
            (1, 2) | (2, 2) | (2, 1) => {
                self.nodes[tail_idx].x += 1;
                self.nodes[tail_idx].y += 1;
            }

            // down-right
            (1, -2) | (2, -2) | (2, -1) => {
                self.nodes[tail_idx].x += 1;
                self.nodes[tail_idx].y -= 1;
            }

            // up-left
            (-1, 2) | (-2, 2) | (-2, 1) => {
                self.nodes[tail_idx].x -= 1;
                self.nodes[tail_idx].y += 1;
            }

            // down-left
            (-1, -2) | (-2, -2) | (-2, -1) => {
                self.nodes[tail_idx].x -= 1;
                self.nodes[tail_idx].y -= 1;
            }

            _ => panic!("Unknown delta ({},{})", d_x, d_y),
        }
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
    let mut s: State<2> = State::new();
    input.iter().for_each(|m| s.do_move(m));
    s.tail_positions()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Move]) -> usize {
    let mut s: State<10> = State::new();
    input.iter().for_each(|m| s.do_move(m));
    s.tail_positions()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_movement_part1() {
        let mut s: State<2> = State::new();

        s.do_move(&Move {
            d: Direction::Up,
            n: 2,
        });
        assert_eq!(s.nodes[0], Position { x: 0, y: 2 });
        assert_eq!(s.nodes[1], Position { x: 0, y: 1 });

        s.nodes[1].y = 2;

        s.do_move(&Move {
            d: Direction::Right,
            n: 2,
        });
        assert_eq!(s.nodes[0], Position { x: 2, y: 2 });
        assert_eq!(s.nodes[1], Position { x: 1, y: 2 });

        s.nodes[1].x = 2;

        s.do_move(&Move {
            d: Direction::Down,
            n: 2,
        });
        assert_eq!(s.nodes[0], Position { x: 2, y: 0 });
        assert_eq!(s.nodes[1], Position { x: 2, y: 1 });

        s.nodes[1].y = 0;

        s.do_move(&Move {
            d: Direction::Left,
            n: 2,
        });
        assert_eq!(s.nodes[0], Position { x: 0, y: 0 });
        assert_eq!(s.nodes[1], Position { x: 1, y: 0 });
    }

    #[test]
    fn diagonal_movement_part1() {
        // first example given
        let mut s: State<2> = State {
            nodes: [Position { x: 2, y: 2 }, Position { x: 1, y: 1 }],
            visited: Default::default(),
        };

        s.do_move(&Move {
            d: Direction::Up,
            n: 1,
        });

        assert_eq!(s.nodes[0], Position { x: 2, y: 3 });
        assert_eq!(s.nodes[1], Position { x: 2, y: 2 });

        // second example given
        s.nodes[0] = Position { x: 2, y: 2 };
        s.nodes[1] = Position { x: 1, y: 1 };

        s.do_move(&Move {
            d: Direction::Right,
            n: 1,
        });

        assert_eq!(s.nodes[0], Position { x: 3, y: 2 });
        assert_eq!(s.nodes[1], Position { x: 2, y: 2 });
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
            1
        );
        assert_eq!(
            solve_part2(&input_generator(
                "R 5\n\
		 U 8\n\
		 L 8\n\
		 D 3\n\
		 R 17\n\
		 D 10\n\
		 L 25\n\
		 U 20"
            )),
            36
        );
    }
}
