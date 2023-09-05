use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum File {
    Directory(Directory),
    Regular(Regular),
}

pub struct Directory {
    name: String,
    files: Vec<Rc<RefCell<File>>>,
    parent: Option<Weak<RefCell<File>>>,
}

pub struct Regular {
    name: String,
    size: usize,
    parent: Weak<RefCell<File>>,
}

impl File {
    fn dir(&mut self) -> &mut Directory {
        match self {
            File::Directory(d) => d,
            _ => panic!("File is not directory"),
        }
    }

    fn reg(&mut self) -> &mut Regular {
        match self {
            File::Regular(r) => r,
            _ => panic!("File is not regular"),
        }
    }

    fn size(&self) -> usize {
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
}

impl Directory {
    fn size(&self) -> usize {
        self.files.iter().map(|f| f.borrow().size()).sum()
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
        name: "/".to_string(),
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
                                name: d.to_string(),
                                files: Vec::new(),
                                parent: Some(Rc::downgrade(&cur)),
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
                let name = parts[1].to_string();

                let this = Rc::new(RefCell::new(File::Regular(Regular {
                    parent: Rc::downgrade(&cur),
                    name,
                    size,
                })));

                cur.borrow_mut().dir().files.push(this);
            }
        }
    }

    root
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Rc<RefCell<File>>) -> usize {
    let mut sum = 0;
    input.borrow().size_filter(&|size| size <= 100000, &mut sum);
    sum
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
}
