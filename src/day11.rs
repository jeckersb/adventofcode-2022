type MonkeyOp = Box<dyn Fn(i64) -> i64>;

enum MonkeyOperand {
    Old,
    Literal(i64),
}

enum MonkeyOperator {
    Add,
    Mul,
}

type MonkeyTest = Box<dyn Fn(i64) -> usize>;

pub struct Monkey {
    items: Vec<i64>,
    op: MonkeyOp,
    test: MonkeyTest,
    divisor: i64,
    inspected: usize,
}

impl From<&str> for MonkeyOperand {
    fn from(s: &str) -> Self {
        match s {
            "old" => MonkeyOperand::Old,
            n => MonkeyOperand::Literal(n.parse().unwrap()),
        }
    }
}

impl From<&str> for MonkeyOperator {
    fn from(s: &str) -> Self {
        match s {
            "+" => MonkeyOperator::Add,
            "*" => MonkeyOperator::Mul,
            _ => panic!("Unknown MonkeyOperator {}", s),
        }
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();

        // don't need "Monkey 0:"
        lines.next();

        // items
        let mut parts = lines.next().unwrap().split(':');
        parts.next();
        let items = parts
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // op
        let mut parts = lines.next().unwrap().split_whitespace().skip(3);

        let lhs = MonkeyOperand::from(parts.next().unwrap());
        let operator = MonkeyOperator::from(parts.next().unwrap());
        let rhs = MonkeyOperand::from(parts.next().unwrap());

        let op = Box::new(move |n| {
            let lhs = match lhs {
                MonkeyOperand::Old => n,
                MonkeyOperand::Literal(l) => l,
            };
            let rhs = match rhs {
                MonkeyOperand::Old => n,
                MonkeyOperand::Literal(l) => l,
            };

            match operator {
                MonkeyOperator::Add => lhs + rhs,
                MonkeyOperator::Mul => lhs * rhs,
            }
        });

        //test
        let divisor = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let true_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let false_monkey = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let test = Box::new(move |n| {
            if n % divisor == 0 {
                true_monkey
            } else {
                false_monkey
            }
        });

        Self {
            items,
            op,
            test,
            divisor,
            inspected: 0,
        }
    }
}

impl Monkey {
    fn take_turn(&mut self, worry_op: &dyn Fn(i64) -> i64) -> Vec<(usize, i64)> {
        std::mem::take(&mut self.items)
            .into_iter()
            .map(|m| {
                self.inspected += 1;
                let level = (self.op)(m);
                let level = worry_op(level);
                let dest = (self.test)(level);
                (dest, level)
            })
            .collect()
    }
}

// #[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut monkeys = input_generator(input);

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            for (i, v) in monkeys[m].take_turn(&|n| n / 3) {
                monkeys[i].items.push(v);
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspected.sort();
    let mut max_iter = inspected.iter().rev();

    max_iter.next().unwrap() * max_iter.next().unwrap()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut monkeys = input_generator(input);

    let modulo: i64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            for (i, v) in monkeys[m].take_turn(&|n| n % modulo) {
                monkeys[i].items.push(v);
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    inspected.sort();
    let mut max_iter = inspected.iter().rev();

    max_iter.next().unwrap() * max_iter.next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn monkey_parse() {
        const MONKEY: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let m = Monkey::from(MONKEY);

        assert_eq!(m.items, vec![79, 98]);
        assert_eq!((m.op)(10), 190);
        assert_eq!((m.test)(23), 2);
        assert_eq!((m.test)(42), 3);
    }

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT), 10605);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT), 2713310158);
    }
}
