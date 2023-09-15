pub enum Instruction {
    Nop,
    Addx(i64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s == "noop" {
            return Self::Nop;
        }

        let mut split = s.split(' ');

        match (split.next().unwrap(), split.next().unwrap()) {
            ("addx", n) => Self::Addx(n.parse().unwrap()),
            (other, _) => panic!("Unknown instruction {}", other),
        }
    }
}

struct Cpu<'a> {
    instructions: &'a [Instruction],
    next_instruction: usize,
    cycle: usize,
    cycles_remaining: usize,
    x: i64,
}

impl<'a> Cpu<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            next_instruction: 0,
            cycle: 0,
            cycles_remaining: 0,
            x: 1,
        }
    }
}

impl<'a> Iterator for Cpu<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        // fetch next instruction?
        if self.cycles_remaining == 0 {
            let instruction = self.instructions.get(self.next_instruction)?;

            self.next_instruction += 1;

            match instruction {
                Instruction::Nop => self.cycles_remaining = 1,
                Instruction::Addx(_) => self.cycles_remaining = 2,
            }
        }

        let ret = self.x;

        self.cycles_remaining -= 1;

        if self.cycles_remaining == 0 {
            if let Instruction::Addx(x) = &self.instructions[self.next_instruction - 1] {
                self.x += x;
            }
        }

        self.cycle += 1;

        Some(ret)
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Instruction]) -> i64 {
    let wanted = [20, 60, 100, 140, 180, 220];
    Cpu::new(input)
        .enumerate()
        .filter_map(|(i, v)| wanted.contains(&(i + 1)).then_some((i + 1) as i64 * v))
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Instruction]) -> String {
    let mut output = String::new();
    let cpu = Cpu::new(input);

    for (i, v) in cpu.enumerate() {
        let idx = i as i64 % 40;
        if (v - 1..=v + 1).contains(&idx) {
            output.push('#')
        } else {
            output.push('.')
        }

        if idx == 39 {
            output.push('\n')
        }
    }

    // remove one trailing newline
    output.pop();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn examples_part1() {
        let short_example = input_generator("noop\naddx 3\naddx -5");
        let mut cpu = Cpu::new(&short_example);
        assert_eq!(cpu.next().unwrap(), 1);
        assert_eq!(cpu.next().unwrap(), 1);
        assert_eq!(cpu.next().unwrap(), 1);
        assert_eq!(cpu.next().unwrap(), 4);
        assert_eq!(cpu.next().unwrap(), 4);

        let example = input_generator(EXAMPLE_INPUT);
        let cpu = Cpu::new(&example);

        // cpu.enumerate()
        //    .for_each(|(i, v)| println!("{}: {v}", i + 1));

        let mut cpu = cpu.skip(19);
        assert_eq!(cpu.next().unwrap() * 20, 420);

        let mut cpu = cpu.skip(39);
        assert_eq!(cpu.next().unwrap() * 60, 1140);

        let mut cpu = cpu.skip(39);
        assert_eq!(cpu.next().unwrap() * 100, 1800);

        let mut cpu = cpu.skip(39);
        assert_eq!(cpu.next().unwrap() * 140, 2940);

        let mut cpu = cpu.skip(39);
        assert_eq!(cpu.next().unwrap() * 180, 2880);

        let mut cpu = cpu.skip(39);
        assert_eq!(cpu.next().unwrap() * 220, 3960);

        assert_eq!(solve_part1(&input_generator(EXAMPLE_INPUT)), 13140);
    }

    #[test]
    fn examples_part2() {
        const RENDERED_EXAMPLE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        assert_eq!(
            solve_part2(&input_generator(EXAMPLE_INPUT)),
            RENDERED_EXAMPLE
        );
    }
}
