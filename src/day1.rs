use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut all = Vec::new();
    let mut cur = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            all.push(cur);
            cur = Vec::new();
        } else {
            cur.push(line.parse().unwrap());
        }
    }

    all.push(cur);
    all
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let mut heap: BinaryHeap<u32> = BinaryHeap::new();

    for elf in input.iter() {
        heap.push(elf.iter().sum());
    }

    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "1000\n\
		 2000\n\
		 3000\n\
		 \n\
		 4000\n\
		 \n\
		 5000\n\
		 6000\n\
		 \n\
		 7000\n\
		 8000\n\
		 9000\n\
		 \n\
		 10000"
            )),
            24000
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "1000\n\
		 2000\n\
		 3000\n\
		 \n\
		 4000\n\
		 \n\
		 5000\n\
		 6000\n\
		 \n\
		 7000\n\
		 8000\n\
		 9000\n\
		 \n\
		 10000"
            )),
            45000
        );
    }
}
