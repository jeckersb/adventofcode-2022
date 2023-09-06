use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum File {
    Directory(Directory),
    Regular(Regular),
}

pub struct Directory {
    _name: String,
    files: Vec<Rc<RefCell<File>>>,
    parent: Option<Weak<RefCell<File>>>,
    size: Option<usize>,
}

pub struct Regular {
    _name: String,
    size: usize,
}

impl File {
    fn dir(&mut self) -> &mut Directory {
        match self {
            File::Directory(d) => d,
            _ => panic!("File is not directory"),
        }
    }

    fn size(&mut self) -> usize {
        match self {
            File::Directory(d) => d.size(),
            File::Regular(f) => f.size(),
        }
    }

    fn size_filter(&self, pred: &impl Fn(usize) -> bool, sum: &mut usize) -> usize {
        match self {
            File::Directory(d) => d.size_filter(pred, sum),
            File::Regular(f) => f.size(),
        }
    }

    fn size_filter_append(&self, pred: &impl Fn(usize) -> bool, acc: &mut Vec<usize>) {
        match self {
            File::Directory(d) => d.size_filter_append(pred, acc),
            File::Regular(_) => {}
        }
    }
}

impl Directory {
    fn size(&mut self) -> usize {
        match self.size {
            Some(s) => s,
            None => {
                let size = self.files.iter().map(|f| f.borrow_mut().size()).sum();
                self.size = Some(size);
                size
            }
        }
    }

    fn size_filter(&self, pred: &impl Fn(usize) -> bool, sum: &mut usize) -> usize {
        let size = self
            .files
            .iter()
            .map(|f| f.borrow().size_filter(pred, sum))
            .sum();
        if pred(size) {
            *sum += size;
        }
        size
    }

    fn size_filter_append(&self, pred: &impl Fn(usize) -> bool, acc: &mut Vec<usize>) {
        let size = self.size.unwrap();

        if pred(size) {
            acc.push(size);
        }

        self.files.iter().for_each(|f| match *f.borrow() {
            File::Directory(ref d) => d.size_filter_append(pred, acc),
            File::Regular(_) => {}
        })
    }
}

impl Regular {
    fn size(&self) -> usize {
        self.size
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Rc<RefCell<File>> {
    let files = Vec::new();
    let parent = None;

    let root = Rc::new(RefCell::new(File::Directory(Directory {
        _name: "/".to_string(),
        size: None,
        files,
        parent,
    })));

    let mut in_ls = false;
    let mut cur = Rc::clone(&root);

    for line in input.lines().skip(1) {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        match parts[0] {
            "$" => {
                if in_ls {
                    in_ls = false;
                }

                match parts[1] {
                    "cd" => match parts[2] {
                        ".." => {
                            let next_cur = cur
                                .borrow_mut()
                                .dir()
                                .parent
                                .as_ref()
                                .unwrap()
                                .upgrade()
                                .unwrap();

                            cur = next_cur;
                        }

                        d => {
                            let subdir = Rc::new(RefCell::new(File::Directory(Directory {
                                _name: d.to_string(),
                                files: Vec::new(),
                                parent: Some(Rc::downgrade(&cur)),
                                size: None,
                            })));

                            let next_cur = Rc::clone(&subdir);
                            cur.borrow_mut().dir().files.push(subdir);
                            cur = next_cur;
                        }
                    },
                    "ls" => {
                        in_ls = true;
                    }
                    other => {
                        panic!("Unknown command {other}")
                    }
                }
            }
            "dir" => { /*nop, we'll catch it when we `cd` */ }
            n => {
                let size = n.parse::<usize>().unwrap();
                let _name = parts[1].to_string();

                let this = Rc::new(RefCell::new(File::Regular(Regular { _name, size })));

                cur.borrow_mut().dir().files.push(this);
            }
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Rc<RefCell<File>>) -> usize {
    let _ = input.borrow_mut().size();
    let mut sum = 0;
    input.borrow().size_filter(&|size| size <= 100000, &mut sum);
    sum
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Rc<RefCell<File>>) -> usize {
    const TOTAL: usize = 70000000;
    const NEEDED: usize = 30000000;
    let mut root = input.borrow_mut();
    let used = root.size();

    let mut candidates = vec![];

    root.size_filter_append(&|n| TOTAL - used + n >= NEEDED, &mut candidates);
    *candidates.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "$ cd /\n\
		 $ ls\n\
		 dir a\n\
		 14848514 b.txt\n\
		 8504156 c.dat\n\
		 dir d\n\
		 $ cd a\n\
		 $ ls\n\
		 dir e\n\
		 29116 f\n\
		 2557 g\n\
		 62596 h.lst\n\
		 $ cd e\n\
		 $ ls\n\
		 584 i\n\
		 $ cd ..\n\
		 $ cd ..\n\
		 $ cd d\n\
		 $ ls\n\
		 4060174 j\n\
		 8033020 d.log\n\
		 5626152 d.ext\n\
		 7214296 k"
            )),
            95437
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "$ cd /\n\
		 $ ls\n\
		 dir a\n\
		 14848514 b.txt\n\
		 8504156 c.dat\n\
		 dir d\n\
		 $ cd a\n\
		 $ ls\n\
		 dir e\n\
		 29116 f\n\
		 2557 g\n\
		 62596 h.lst\n\
		 $ cd e\n\
		 $ ls\n\
		 584 i\n\
		 $ cd ..\n\
		 $ cd ..\n\
		 $ cd d\n\
		 $ ls\n\
		 4060174 j\n\
		 8033020 d.log\n\
		 5626152 d.ext\n\
		 7214296 k"
            )),
            24933642
        );
    }
}
