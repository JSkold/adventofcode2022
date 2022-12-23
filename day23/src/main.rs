use std::{
    cell::RefCell,
    cmp::{max, min},
    collections::{HashMap, HashSet},
    io::stdin,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn check_tiles(self) -> impl Iterator<Item = (isize, isize)> {
        match self {
            Direction::North => vec![(0, -1), (1, -1), (-1, -1)].into_iter(),
            Direction::South => vec![(0, 1), (1, 1), (-1, 1)].into_iter(),
            Direction::West => vec![(-1, 0), (-1, -1), (-1, 1)].into_iter(),
            Direction::East => vec![(1, 0), (1, -1), (1, 1)].into_iter(),
        }
    }

    fn move_tile(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

fn direction_iterator() -> impl Iterator<Item = Direction> {
    vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .cycle()
}

// Did this because I was sure the elves were going to keep track of their own
// rotation for task 2.
struct Elf {
    dir_it: RefCell<Box<dyn Iterator<Item = Direction>>>,
}

fn main() {
    let mut elves: HashMap<(isize, isize), Elf> = HashMap::new();

    let mut lines = stdin().lines().enumerate();
    while let Some((y, Ok(line))) = lines.next() {
        for (x, c) in line.char_indices() {
            match c {
                '#' => {
                    elves.insert(
                        (x as isize, y as isize),
                        Elf {
                            dir_it: RefCell::new(Box::new(direction_iterator())),
                        },
                    );
                }
                '.' => continue,
                _ => unimplemented!(),
            }
        }
    }

    let mut i = 0;
    loop {
        i += 1;

        // Key is proposed position, value is current position.
        let mut proposition_set: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
        let mut collision_set: HashSet<(isize, isize)> = HashSet::new();

        for (pos, elf) in elves.iter() {
            // Grab 5 elements from the direction iterator. Each elf needs to consider all 4 directions
            // and we want to offset so that they begin looking at the next one next round. The first and
            // fifth direction will have the same outcome anyway so this is safe. We are also forced to use
            // Iterator::next here since we only have a reference to the iterator which limit what we can do.
            let mut borrowed_it = elf.dir_it.borrow_mut();
            let dirs: Vec<Direction> = vec![
                borrowed_it.next().unwrap(),
                borrowed_it.next().unwrap(),
                borrowed_it.next().unwrap(),
                borrowed_it.next().unwrap(),
                borrowed_it.next().unwrap(),
            ];
            // If elves in any adjesent tiles. (Diagonal positions are checked twice, cba to fix.)
            if dirs.iter().any(|d| {
                d.check_tiles()
                    .any(|p| elves.contains_key(&(pos.0 + p.0, pos.1 + p.1)))
            }) {
                for dir in dirs {
                    if dir
                        .check_tiles()
                        .all(|p| !elves.contains_key(&(pos.0 + p.0, pos.1 + p.1)))
                    {
                        let pos_offset = dir.move_tile();
                        let proposed_pos = (pos.0 + pos_offset.0, pos.1 + pos_offset.1);
                        if proposition_set
                            .insert(proposed_pos.clone(), pos.clone())
                            .is_some()
                        {
                            collision_set.insert(proposed_pos);
                        }
                        break;
                    }
                }
            }
        }

        // Filter out all positions that are present in the collision set. Only acceptable
        // positions are now left.
        proposition_set.retain(|k, _| !collision_set.contains(k));

        // Break if no more moves, Task 2.
        if proposition_set.len() == 0 {
            println!("{}", i);
            break;
        }

        // Preform moves
        for (next, cur) in proposition_set {
            // Safe unwrap
            let elf = elves.remove(&cur).unwrap();
            elves.insert(next, elf);
        }

        // Task 1
        if i == 10 {
            let num_elves = elves.len();
            let mut min_x = isize::MAX;
            let mut max_x = isize::MIN;
            let mut min_y = isize::MAX;
            let mut max_y = isize::MIN;
            for (p, _) in elves.iter() {
                min_x = min(p.0, min_x);
                max_x = max(p.0, max_x);
                min_y = min(p.1, min_y);
                max_y = max(p.1, max_y);
            }

            println!(
                "{}",
                ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - num_elves
            );
        }
    }
}
