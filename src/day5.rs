struct Crates(Vec<Vec<char>>);

struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

struct Puzzle {
    crates: Crates,
    moves: Vec<Move>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let parts: Vec<_> = input.split("\n\n").collect();

        assert_eq!(parts.len(), 2);

        let crates = Crates::from(parts[0]);
        let moves = parts[1].lines().map(Move::from).collect();

        Self { crates, moves }
    }
}

impl From<&str> for Crates {
    fn from(input: &str) -> Self {
        let mut lines: Vec<_> = input.lines().collect();

        // ignore the labels at the end
        lines.pop();

        let n = Crates::num_crates(lines[0].len());
        let mut stacks = Vec::with_capacity(n);

        for _ in 0..n {
            stacks.push(Vec::new());
        }

        for &line in lines.iter().rev() {
            let mut chars = line.chars();

            for stack in stacks.iter_mut() {
                chars.next(); // '['

                // letter?
                let ch = chars.next().unwrap();
                if ch.is_alphabetic() {
                    stack.push(ch);
                }

                chars.next(); // ']'
                chars.next(); // ' '
            }
        }

        Self(stacks)
    }
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let parts: Vec<_> = input.split_whitespace().collect();

        assert_eq!(parts.len(), 6);

        let qty = parts[1].parse().unwrap();
        let from = (parts[3].parse::<isize>().unwrap() - 1) as usize;
        let to = (parts[5].parse::<isize>().unwrap() - 1) as usize;

        Self { qty, from, to }
    }
}

impl Crates {
    fn num_crates(len: usize) -> usize {
        (len + 1) / 4
    }

    fn top_crates(&self) -> Vec<char> {
        let mut vec = Vec::new();

        for stack in self.0.iter() {
            if let Some(&ch) = stack.last() {
                vec.push(ch);
            }
        }

        vec
    }

    fn mv(&mut self, from: usize, to: usize) {
        let tmp = (self.0)[from].pop().unwrap();
        (self.0)[to].push(tmp);
    }

    fn mv_stack(&mut self, from: usize, to: usize, qty: usize) {
        let mut tmp = Vec::with_capacity(qty);

        for _ in 0..qty {
            tmp.push((self.0)[from].pop().unwrap());
        }

        while let Some(ch) = tmp.pop() {
            (self.0)[to].push(ch);
        }
    }
}

impl Puzzle {
    fn process(&mut self) {
        for mv in self.moves.iter() {
            for _ in 0..mv.qty {
                self.crates.mv(mv.from, mv.to)
            }
        }
    }

    fn process2(&mut self) {
        for mv in self.moves.iter() {
            self.crates.mv_stack(mv.from, mv.to, mv.qty)
        }
    }

    fn top_crates(&self) -> Vec<char> {
        self.crates.top_crates()
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> String {
    let mut puzzle = Puzzle::from(input);
    puzzle.process();
    puzzle.top_crates().into_iter().collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> String {
    let mut puzzle = Puzzle::from(input);
    puzzle.process2();
    puzzle.top_crates().into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(
                "    [D]    \n\
		 [N] [C]    \n\
		 [Z] [M] [P]\n\
		  1   2   3 \n\
		 \n\
		 move 1 from 2 to 1\n\
		 move 3 from 1 to 3\n\
		 move 2 from 2 to 1\n\
		 move 1 from 1 to 2"
            ),
            "CMZ"
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(
                "    [D]    \n\
		 [N] [C]    \n\
		 [Z] [M] [P]\n\
		  1   2   3 \n\
		 \n\
		 move 1 from 2 to 1\n\
		 move 3 from 1 to 3\n\
		 move 2 from 2 to 1\n\
		 move 1 from 1 to 2"
            ),
            "MCD"
        );
    }
}
