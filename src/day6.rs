use std::collections::HashSet;

fn solve(input: &str, size: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();

    chars
        .windows(size)
        .enumerate()
        .find(|(_i, win)| win.len() == win.iter().copied().collect::<HashSet<char>>().len())
        .unwrap()
        .0
        + size
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    solve(input, 4)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    solve(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
