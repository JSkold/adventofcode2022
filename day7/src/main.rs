use std::{cell::RefCell, collections::HashMap, io::stdin, rc::Rc};

enum Ent {
    File { file: File },
    Dir { dir: Rc<RefCell<Dir>> },
}

struct File {
    size: u32,
}

struct Dir {
    sub: HashMap<String, Ent>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Ent {
    fn get_size(&self) -> u32 {
        match self {
            Ent::File { file } => file.size,
            Ent::Dir { dir } => dir.borrow().sub.values().map(|e| e.get_size()).sum(),
        }
    }

    fn sum_dir_below_size(&self, at_most: u32) -> u32 {
        let self_size = self.get_size();
        let mut tot_size = 0;
        tot_size += if self_size <= at_most { self_size } else { 0 };

        match self {
            Ent::File { file: _ } => return 0,
            Ent::Dir { dir } => {
                tot_size += dir
                    .borrow()
                    .sub
                    .values()
                    .map(|e| e.sum_dir_below_size(at_most))
                    .sum::<u32>()
            }
        };
        tot_size
    }

    fn smallest_dir_at_least(&self, at_least: u32) -> u32 {
        let self_size = match self.get_size().checked_sub(at_least) {
            Some(s) => s,
            None => u32::MAX - at_least,
        } + at_least;

        let min_dir = match self {
            Ent::File { file: _ } => return u32::MAX,
            Ent::Dir { dir } => dir
                .borrow()
                .sub
                .values()
                .map(|e| e.smallest_dir_at_least(at_least))
                .min()
                .unwrap(),
        };
        std::cmp::min(self_size, min_dir)
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Dir {
        sub: HashMap::new(),
        parent: None,
    }));
    let mut current_dir = root.clone();

    let mut lines = stdin().lines().peekable();
    // Skip initial $ cd /
    lines.next();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with('$') {
            let mut cmd_split = line.split(' ');
            match cmd_split.nth(1).unwrap() {
                "cd" => match cmd_split.next().unwrap() {
                    ".." => {
                        // Move dir up
                        let up = current_dir.borrow().parent.as_ref().unwrap().clone();
                        current_dir = up;
                    }
                    s => {
                        // Move dir down
                        let down = match current_dir.borrow().sub.get(s).unwrap() {
                            Ent::File { file: _ } => unimplemented!(),
                            Ent::Dir { dir } => dir.clone(),
                        };
                        current_dir = down;
                    }
                },
                "ls" => {
                    while let Some(p) = lines.peek() {
                        if !p.as_ref().unwrap().starts_with('$') {
                            let line_entry = lines.next().unwrap().unwrap();
                            let line_entry_split = &mut line_entry.split(' ');
                            match line_entry_split.next().unwrap() {
                                "dir" => {
                                    // Add directory entry
                                    let name = line_entry_split.next().unwrap();
                                    let new = Dir {
                                        sub: HashMap::new(),
                                        parent: Some(current_dir.clone()),
                                    };
                                    current_dir.borrow_mut().sub.insert(
                                        name.to_string(),
                                        Ent::Dir {
                                            dir: Rc::new(RefCell::new(new)),
                                        },
                                    );
                                }
                                s => {
                                    // Add file entry
                                    let name = line_entry_split.next().unwrap();
                                    current_dir.borrow_mut().sub.insert(
                                        name.to_string(),
                                        Ent::File {
                                            file: File {
                                                size: s.parse::<u32>().unwrap(),
                                            },
                                        },
                                    );
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    // Solution for part 1
    println!(
        "{}",
        Ent::Dir { dir: root.clone() }.sum_dir_below_size(100000)
    );

    // Solution for part 2
    let root_size = Ent::Dir { dir: root.clone() }.get_size();
    let needed_space = 30000000 + root_size - 70000000;
    println!(
        "{}",
        Ent::Dir { dir: root.clone() }.smallest_dir_at_least(needed_space)
    );
}
