#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub struct Round {
    elf: Choice,
    own: Choice,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Choice {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn versus(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissors, Choice::Paper) => Outcome::Win,
            _ => Outcome::Loss,
        }
    }
}

impl From<char> for Choice {
    fn from(ch: char) -> Self {
        match ch {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            other => panic!("Unknown choice '{}'", other),
        }
    }
}

impl Round {
    fn score(&self) -> u32 {
        self.own.score() + self.own.versus(&self.elf).score()
    }

    fn part2(&self) -> Self {
        let desired_outcome = Outcome::from(self.own);

        let own = match (self.elf, desired_outcome) {
            (Choice::Scissors, Outcome::Win)
            | (Choice::Rock, Outcome::Draw)
            | (Choice::Paper, Outcome::Loss) => Choice::Rock,
            (Choice::Rock, Outcome::Win)
            | (Choice::Paper, Outcome::Draw)
            | (Choice::Scissors, Outcome::Loss) => Choice::Paper,
            _ => Choice::Scissors,
        };

        Self { elf: self.elf, own }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl From<Choice> for Outcome {
    fn from(choice: Choice) -> Self {
        match choice {
            Choice::Rock => Self::Loss,
            Choice::Paper => Self::Draw,
            Choice::Scissors => Self::Win,
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    let mut rounds = Vec::new();

    for bytes in input.as_bytes().chunks(4) {
        rounds.push(Round {
            elf: Choice::from(bytes[0] as char),
            own: Choice::from(bytes[2] as char),
        })
    }

    rounds
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Round]) -> u32 {
    input.iter().map(|round| round.score()).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Round]) -> u32 {
    input.iter().map(|round| round.part2().score()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "A Y\n\
		 B X\n\
		 C Z\n"
            )),
            15
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "A Y\n\
		 B X\n\
		 C Z\n"
            )),
            12
        );
    }
}
