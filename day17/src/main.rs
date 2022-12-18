#![feature(iter_collect_into)]
#![feature(try_blocks)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    io::stdin,
    iter,
};

#[derive(Clone, Copy, Debug)]
enum Gas {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Material {
    Air,
    Rock,
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}

impl Into<char> for &Material {
    fn into(self) -> char {
        match self {
            Material::Air => '.',
            Material::Rock => '#',
        }
    }
}

struct Chamber {
    space: VecDeque<Material>,
    // Simulated rock level
    rock_level: u64,
    // Rock level in space
    rock_mem_level: usize,
}

impl Chamber {
    const SPAWN_HEIGHT: usize = 3;
    const SPAWN_POSITION: usize = 2;
    const WIDTH: usize = 7;

    fn new() -> Self {
        let space: VecDeque<Material> = VecDeque::new();

        Chamber {
            space,
            rock_level: 0,
            rock_mem_level: 0,
        }
    }

    fn expand_space(&mut self, lines: usize) {
        if lines == 0 {
            return;
        }
        self.space
            .extend(iter::repeat(Material::Air).take(lines * Chamber::WIDTH));
        self.rock_mem_level += lines;
    }

    #[allow(dead_code)]
    fn truncate_space(&mut self, lines_to_remove: usize) {
        drop(self.space.drain(0..(lines_to_remove * Chamber::WIDTH)));
        self.rock_mem_level -= lines_to_remove;
    }

    fn get_pos(&self, x: usize, y: usize) -> Material {
        self.space[y * Chamber::WIDTH + x]
    }

    fn set_pos(&mut self, x: usize, y: usize, mat: Material) {
        self.space[y * Chamber::WIDTH + x] = mat;
    }

    fn settle_boulder(&mut self, x: usize, y: usize, boulder: BoulderType) {
        let relative_top = y + boulder.height_fast();

        self.rock_level += relative_top.saturating_sub(self.rock_mem_level) as u64;

        self.expand_space(relative_top.saturating_sub(self.rock_mem_level));

        let rock_pos = boulder.relative_rock_positions_fast();

        for (rx, ry) in rock_pos {
            self.set_pos(x + rx, y + ry, Material::Rock);
        }

        // The input makes sure to never fill all blocks in a row, what the hell!?
        // Try to find a path from one side of the chamber to the other.
        // Anything below the lowest point of that path is inaccessable.

        // Only run if the settled block landed on the edge and we already have a large backlog
        if x == 0 && y > 10_000_000 {
            // Recycle solution from day 12
            #[allow(dead_code)]
            fn path_find(c: &Chamber, x: usize, y: usize) -> Option<usize> {
                let mut paths: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
                let candidate_pos = vec![
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                ];

                paths.insert((x, y), vec![]);

                loop {
                    let mut new_steps: HashMap<(usize, usize), Vec<(usize, usize)>> =
                        HashMap::new();
                    let mut paths_it = paths.iter();
                    while let Some(((new_x, new_y), path)) = paths_it.next() {
                        let steps: Vec<(usize, usize)> = candidate_pos
                            .iter()
                            .filter_map(|(sx, sy)| try {
                                (
                                    new_x.checked_add_signed(*sx)?,
                                    new_y.checked_add_signed(*sy)?,
                                )
                            })
                            .filter(|(sx, sy)| *sx < Chamber::WIDTH && *sy < c.rock_mem_level)
                            .filter(|s| !paths.contains_key(s))
                            .collect();

                        for step in steps {
                            // Attempt to break early
                            if step.0 == Chamber::WIDTH - 1 {
                                return path.into_iter().map(|(_, py)| py).min().cloned();
                            }

                            let mut new_path = path.clone();
                            new_path.push(step.clone());
                            new_steps.insert(step.clone(), new_path);
                        }
                    }

                    if new_steps.len() == 0 {
                        break;
                    }
                    new_steps
                        .drain()
                        .for_each(|(k, v)| assert!(paths.insert(k, v).is_none()));
                }

                None
            }

            // I can't get a deterministic output using this method of memory discarding.
            // I give up, the cycle finding method will find a solution before memory becomes an issue.

            // I'm not crazy thinking this should work right?
            // Lowest point of a path between left and right side of chamber?

            //match path_find(self, x, y) {
            //    Some(low) => {
            //        self.truncate_space(low);
            //    }
            //    None => {}
            //}
        }
    }

    fn will_settle(&self, x: usize, y: usize, boulder: BoulderType) -> bool {
        let rock_pos = boulder.relative_rock_positions_fast();

        if y == 0 {
            return true;
        }

        for (rx, ry) in rock_pos {
            // Don't bother checking out of bounds
            if y + ry > self.rock_mem_level {
                continue;
            }
            if self.get_pos(x + rx, y + ry - 1) == Material::Rock {
                return true;
            }
        }
        return false;
    }

    fn unobstructed_movement(x: usize, gas: &Gas, boulder: BoulderType) -> usize {
        match gas {
            Gas::Left => x.saturating_sub(1),
            Gas::Right => {
                if x + boulder.width_fast() >= Chamber::WIDTH {
                    x
                } else {
                    x + 1
                }
            }
        }
    }

    fn obstructed_movement(&self, x: usize, y: usize, gas: &Gas, boulder: BoulderType) -> usize {
        match gas {
            Gas::Left => {
                if x == 0 {
                    return x;
                }
                for (rx, ry) in boulder.relative_rock_positions_fast() {
                    if y + ry >= self.rock_mem_level {
                        continue;
                    }
                    if self.get_pos(x + rx - 1, y + ry) == Material::Rock {
                        return x;
                    }
                }
                x - 1
            }
            Gas::Right => {
                for (rx, ry) in boulder.relative_rock_positions_fast() {
                    if rx + x + 1 >= Chamber::WIDTH {
                        return x;
                    }
                    if y + ry >= self.rock_mem_level {
                        continue;
                    }
                    if self.get_pos(x + rx + 1, y + ry) == Material::Rock {
                        return x;
                    }
                }
                x + 1
            }
        }
    }

    fn fall_boulder(&mut self, gas: &mut dyn Iterator<Item = Gas>, boulder: BoulderType) {
        let mut x = Chamber::SPAWN_POSITION;
        let mut y = self.rock_mem_level;

        // The rock cannot settle before being pushed three times by gas
        for _ in 0..=Chamber::SPAWN_HEIGHT {
            x = Chamber::unobstructed_movement(x, &gas.next().unwrap(), boulder);
        }

        if !self.will_settle(x, y, boulder) {
            loop {
                y -= 1;
                x = self.obstructed_movement(x, y, &gas.next().unwrap(), boulder);
                // Check is allowed to move a step down
                if self.will_settle(x, y, boulder) {
                    break;
                }
            }
        }

        self.settle_boulder(x, y, boulder);
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.rock_mem_level).rev() {
            for x in 0..Chamber::WIDTH {
                write!(f, "{}", Into::<char>::into(&self.get_pos(x, y)))?
            }
            write!(f, "  {}\n", y)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
enum BoulderType {
    Minus,
    Plus,
    InvertedL,
    Line,
    Square,
}

impl BoulderType {
    fn relative_rock_positions_fast(&self) -> &[(usize, usize)] {
        match self {
            BoulderType::Minus => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            BoulderType::Plus => &[(1, 0), (0, 1), (2, 1), (1, 1), (1, 2)],
            BoulderType::InvertedL => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            BoulderType::Line => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            BoulderType::Square => &[(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn width_fast(&self) -> usize {
        match self {
            BoulderType::Minus => 4,
            BoulderType::Plus => 3,
            BoulderType::InvertedL => 3,
            BoulderType::Line => 1,
            BoulderType::Square => 2,
        }
    }

    fn height_fast(&self) -> usize {
        match self {
            BoulderType::Minus => 1,
            BoulderType::Plus => 3,
            BoulderType::InvertedL => 3,
            BoulderType::Line => 4,
            BoulderType::Square => 2,
        }
    }
}

fn find_cycle(samples: &Vec<usize>) -> Option<(usize, usize)> {
    for i in 1.. {
        let mut chunks = samples.chunks_exact(i);

        let first = chunks.next()?;
        if chunks.len() >= 3 && chunks.all(|c| c == first) {
            return Some((first.into_iter().sum(), i));
        }
    }
    None
}

fn main() {
    let mut gas: Vec<Gas> = Vec::new();
    let boulders: Vec<BoulderType> = vec![
        BoulderType::Minus,
        BoulderType::Plus,
        BoulderType::InvertedL,
        BoulderType::Line,
        BoulderType::Square,
    ];

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        line.chars()
            .map(|c| match c {
                '<' => Gas::Left,
                '>' => Gas::Right,
                _ => unimplemented!(),
            })
            .collect_into(&mut gas);
    }
    let gas_length = gas.len();
    let boulder_length = boulders.len();
    let mut gas = gas.into_iter().cycle();
    let mut boulders = boulders.iter().cycle();

    let mut chamber = Chamber::new();

    let mut adjust = 0;
    let mut prev_cycle: usize = 0;
    let mut num_cycles: usize = 0;
    let mut samples: Vec<usize> = Vec::new();
    let mut done = false;

    let mut i = 0;
    loop {
        if i >= 1_000_000_000_000 {
            break;
        }
        if i % (gas_length * boulder_length) as u64 == 0 {
            num_cycles += 1;

            // Lets run a couple of cycles before sampling to make sure we aren't
            // affected by initial condition
            if num_cycles > 10 {
                samples.push(chamber.rock_level as usize - prev_cycle);
            }
            prev_cycle = chamber.rock_level as usize;

            if !done && num_cycles > 50 {
                match find_cycle(&samples) {
                    Some((diff, size)) => {
                        done = true;
                        let simulated_cycles = (1_000_000_000_000 - i)
                            / (size as u64 * (gas_length * boulder_length) as u64);

                        i += simulated_cycles * size as u64 * (gas_length * boulder_length) as u64;
                        adjust = simulated_cycles * diff as u64;
                    }
                    None => {}
                }
            }
        }

        // Task 1
        if i == 2022 {
            println!("{}", chamber.rock_level);
        }
        chamber.fall_boulder(&mut gas, *boulders.next().unwrap());
        i += 1;
    }

    println!("{}", chamber.rock_level + adjust);
}
