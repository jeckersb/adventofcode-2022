use itertools::Itertools;

pub struct Map {
    inner: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

pub struct MapCell<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

pub struct MapIter<'a> {
    map: &'a Map,
    next_x: usize,
    next_y: usize,
}

impl Map {
    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let cur = self.inner[y][x];

        if self.is_edge(x, y) {
            return true;
        }

        // up
        if (0..y).all(|y| self.inner[y][x] < cur) {
            return true;
        }

        // down
        if (y + 1..self.height).all(|y| self.inner[y][x] < cur) {
            return true;
        }

        // left
        if (0..x).all(|x| self.inner[y][x] < cur) {
            return true;
        }

        // right
        if (x + 1..self.width).all(|x| self.inner[y][x] < cur) {
            return true;
        }

        false
    }

    fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1
    }

    fn iter(&self) -> MapIter {
        MapIter {
            map: self,
            next_x: 0,
            next_y: 0,
        }
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        if self.is_edge(x, y) {
            return 0;
        }

        let cur = self.inner[y][x];

        let up = (0..y)
            .rev()
            .take_while_inclusive(|y| self.inner[*y][x] < cur)
            .count();

        let down = (y + 1..self.height)
            .take_while_inclusive(|y| self.inner[*y][x] < cur)
            .count();

        let left = (0..x)
            .rev()
            .take_while_inclusive(|x| self.inner[y][*x] < cur)
            .count();

        let right = (x + 1..self.width)
            .take_while_inclusive(|x| self.inner[y][*x] < cur)
            .count();

        up * down * left * right
    }
}

impl<'a> MapCell<'a> {
    fn is_visible(&self) -> bool {
        self.map.is_visible(self.x, self.y)
    }

    fn scenic_score(&self) -> usize {
        self.map.scenic_score(self.x, self.y)
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = MapCell<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y == self.map.height {
            return None;
        }

        if self.next_x == self.map.width {
            self.next_y += 1;
            self.next_x = 0;
            return self.next();
        }

        let cell = MapCell {
            map: self.map,
            x: self.next_x,
            y: self.next_y,
        };

        self.next_x += 1;
        Some(cell)
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    let mut map = Vec::new();

    for (i, line) in input.lines().enumerate() {
        map.push(Vec::new());

        line.chars()
            .for_each(|c| map[i].push(c.to_digit(10).unwrap()));
    }

    let width = map[0].len();
    let height = map.len();

    Map {
        inner: map,
        width,
        height,
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Map) -> usize {
    input.iter().filter(|c| c.is_visible()).count()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Map) -> usize {
    input.iter().map(|c| c.scenic_score()).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "30373\n\
		 25512\n\
		 65332\n\
		 33549\n\
		 35390"
            )),
            21
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "30373\n\
		 25512\n\
		 65332\n\
		 33549\n\
		 35390"
            )),
            8
        );
    }
}
