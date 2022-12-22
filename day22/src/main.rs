use std::{io::stdin, iter::Peekable};

#[derive(PartialEq, Eq)]
enum Tile {
    None,
    Open,
    Solid,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn rotate(&self, r: Movement) -> Self {
        match r {
            Movement::RotateRight => match self {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            },
            Movement::RotateLeft => match self {
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Up => Direction::Left,
            },
            _ => unimplemented!(),
        }
    }

    fn score(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum Movement {
    RotateRight,
    RotateLeft,
    MoveForward(usize),
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::RotateRight,
            'L' => Self::RotateLeft,
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for Movement {
    fn from(value: &str) -> Self {
        assert!(!value.is_empty());
        if let Some(c) = value.chars().next() {
            if c.is_alphabetic() {
                c.into()
            } else {
                Movement::MoveForward(value.parse::<usize>().unwrap())
            }
        } else {
            unimplemented!()
        }
    }
}

fn main() {
    let mut map: Vec<Vec<Tile>> = vec![];
    let mut path_string = String::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        // Empty line between map and path input
        if line.is_empty() {
            path_string = lines.next().unwrap().unwrap();
            break;
        }

        map.push(
            line.chars()
                .map(|c| match c {
                    ' ' => Tile::None,
                    '.' => Tile::Open,
                    '#' => Tile::Solid,
                    _ => unimplemented!(),
                })
                .collect(),
        );
    }

    let mut position = (
        map[0].iter().position(|t| *t == Tile::Open).unwrap(),
        0 as usize,
    );
    let mut position2 = position.clone();
    let mut direction = Direction::Right;
    let mut direction2 = direction;

    for m in path_descriptor(&path_string) {
        match m {
            Movement::RotateRight => {
                direction = direction.rotate(m);
                direction2 = direction2.rotate(m);
            }
            Movement::RotateLeft => {
                direction = direction.rotate(m);
                direction2 = direction2.rotate(m);
            }
            Movement::MoveForward(n) => {
                // Skip 1 for the tile we are currently standing on.
                let path = path_iterator(&map, position, direction).skip(1).take(n);
                for (tile, pos) in path {
                    match tile {
                        Tile::None => unreachable!(),
                        Tile::Open => {
                            position = pos;
                        }
                        Tile::Solid => break,
                    }
                }

                let path2 = cube_path_iterator(&map, position2, direction2)
                    .skip(1)
                    .take(n);

                for (tile, pos, dir) in path2 {
                    match tile {
                        Tile::None => unreachable!(),
                        Tile::Open => {
                            position2 = pos;
                            direction2 = dir;
                        }
                        Tile::Solid => break,
                    }
                }
            }
        }
    }

    println!(
        "{}",
        (position.1 + 1) * 1000 + (position.0 + 1) * 4 + direction.score()
    );
    println!(
        "{}",
        (position2.1 + 1) * 1000 + (position2.0 + 1) * 4 + direction2.score()
    );
}

fn path_descriptor<'a>(string: &'a str) -> impl Iterator<Item = Movement> + 'a {
    string
        .split_inclusive(|c: char| c.is_alphabetic())
        .map(|str| {
            if str.ends_with(|c: char| c.is_alphabetic()) {
                let split = str.split_at(str.len() - 1);
                [split.0.into(), split.1.into()]
                    .into_iter()
                    .collect::<Vec<Movement>>()
            } else {
                [str.into()].into_iter().collect::<Vec<Movement>>()
            }
        })
        .flatten()
}

fn path_iterator<'a>(
    map: &'a Vec<Vec<Tile>>,
    pos: (usize, usize),
    dir: Direction,
) -> Peekable<Box<dyn Iterator<Item = (&Tile, (usize, usize))> + 'a>> {
    match dir {
        Direction::Right => (Box::new(
            map[pos.1]
                .iter()
                .enumerate()
                .cycle()
                .skip(pos.0)
                .filter(|(_, t)| **t != Tile::None)
                .map(move |(i, t)| (t, (i, pos.1))),
        ) as Box<dyn Iterator<Item = (&Tile, (usize, usize))>>)
            .peekable(),
        Direction::Down => (Box::new(
            map.iter()
                .enumerate()
                .cycle()
                .skip(pos.1)
                .map(move |(i, r)| (r.get(pos.0).unwrap_or(&Tile::None), (pos.0, i)))
                .filter(|(t, _)| **t != Tile::None),
        ) as Box<dyn Iterator<Item = (&Tile, (usize, usize))>>)
            .peekable(),
        Direction::Left => (Box::new(
            map[pos.1]
                .iter()
                .enumerate()
                .rev()
                .cycle()
                .skip(map[pos.1].len() - 1 - pos.0)
                .filter(|(_, t)| **t != Tile::None)
                .map(move |(i, t)| (t, (i, pos.1))),
        ) as Box<dyn Iterator<Item = (&Tile, (usize, usize))>>)
            .peekable(),
        Direction::Up => (Box::new(
            map.iter()
                .enumerate()
                .rev()
                .cycle()
                .skip(map.len() - 1 - pos.1)
                .map(move |(i, r)| (r.get(pos.0).unwrap_or(&Tile::None), (pos.0, i)))
                .filter(|(t, _)| **t != Tile::None),
        ) as Box<dyn Iterator<Item = (&Tile, (usize, usize))>>)
            .peekable(),
    }
}

fn cube_map(pos: (usize, usize), dir: Direction) -> ((usize, usize), Direction) {
    const CUBE_SIZE: usize = 50;
    match dir {
        Direction::Right => match pos.1 / CUBE_SIZE {
            0 => {
                let y_offset = CUBE_SIZE - pos.1 - 1;
                (
                    (99, 100 + y_offset),
                    dir.rotate(Movement::RotateRight)
                        .rotate(Movement::RotateRight),
                )
            }
            1 => {
                let x_offset = pos.1 - CUBE_SIZE;
                ((100 + x_offset, 49), dir.rotate(Movement::RotateLeft))
            }
            2 => {
                let y_offset = CUBE_SIZE * 3 - pos.1 - 1;
                (
                    (149, 0 + y_offset),
                    dir.rotate(Movement::RotateLeft)
                        .rotate(Movement::RotateLeft),
                )
            }
            3 => {
                let x_offset = pos.1 - CUBE_SIZE * 3;
                ((50 + x_offset, 149), dir.rotate(Movement::RotateLeft))
            }
            _ => unimplemented!(),
        },
        Direction::Down => match pos.0 / CUBE_SIZE {
            0 => {
                let x_offset = pos.0;
                ((100 + x_offset, 0), dir)
            }
            1 => {
                let y_offset = pos.0 - CUBE_SIZE;
                ((49, 150 + y_offset), dir.rotate(Movement::RotateRight))
            }
            2 => {
                let y_offset = pos.0 - CUBE_SIZE * 2;
                ((99, 50 + y_offset), dir.rotate(Movement::RotateRight))
            }
            _ => unimplemented!(),
        },
        Direction::Left => match pos.1 / CUBE_SIZE {
            0 => {
                let y_offset = CUBE_SIZE - pos.1 - 1;
                (
                    (0, 100 + y_offset),
                    dir.rotate(Movement::RotateLeft)
                        .rotate(Movement::RotateLeft),
                )
            }
            1 => {
                let x_offset = pos.1 - CUBE_SIZE * 1;
                ((0 + x_offset, 100), dir.rotate(Movement::RotateLeft))
            }
            2 => {
                let y_offset = CUBE_SIZE * 3 - pos.1 - 1;
                (
                    (50, 0 + y_offset),
                    dir.rotate(Movement::RotateLeft)
                        .rotate(Movement::RotateLeft),
                )
            }
            3 => {
                let x_offset = pos.1 - CUBE_SIZE * 3;
                ((50 + x_offset, 0), dir.rotate(Movement::RotateLeft))
            }
            _ => unimplemented!(),
        },
        Direction::Up => match pos.0 / CUBE_SIZE {
            0 => {
                let y_offest = pos.0;
                ((50, 50 + y_offest), dir.rotate(Movement::RotateRight))
            }
            1 => {
                let y_offset = pos.0 - CUBE_SIZE;
                ((0, 150 + y_offset), dir.rotate(Movement::RotateRight))
            }
            2 => {
                let x_offset = pos.0 - CUBE_SIZE * 2;
                ((0 + x_offset, 199), dir)
            }
            _ => unimplemented!(),
        },
    }
}

fn wrapless_path_iterator<'a>(
    map: &'a Vec<Vec<Tile>>,
    pos: (usize, usize),
    dir: Direction,
) -> Box<dyn Iterator<Item = (&Tile, (usize, usize), Direction)> + 'a> {
    match dir {
        Direction::Right => Box::new(
            map[pos.1]
                .iter()
                .enumerate()
                .skip(pos.0)
                .filter(|(_, t)| **t != Tile::None)
                .map(move |(i, t)| (t, (i, pos.1), dir)),
        ),
        Direction::Down => Box::new(
            map.iter()
                .enumerate()
                .skip(pos.1)
                .map(move |(i, r)| (r.get(pos.0).unwrap_or(&Tile::None), (pos.0, i), dir))
                .filter(|(t, _, _)| **t != Tile::None),
        ),
        Direction::Left => Box::new(
            map[pos.1]
                .iter()
                .enumerate()
                .rev()
                .skip(map[pos.1].len() - 1 - pos.0)
                .filter(|(_, t)| **t != Tile::None)
                .map(move |(i, t)| (t, (i, pos.1), dir)),
        ),
        Direction::Up => Box::new(
            map.iter()
                .enumerate()
                .rev()
                .skip(map.len() - 1 - pos.1)
                .map(move |(i, r)| (r.get(pos.0).unwrap_or(&Tile::None), (pos.0, i), dir))
                .filter(|(t, _, _)| **t != Tile::None),
        ),
    }
}

fn cube_path_iterator<'a>(
    map: &'a Vec<Vec<Tile>>,
    pos: (usize, usize),
    dir: Direction,
) -> impl Iterator<Item = (&Tile, (usize, usize), Direction)> + 'a {
    let mut path: Vec<(&Tile, (usize, usize), Direction)> = vec![];
    let mut continue_from = (pos, dir);

    loop {
        path.extend(wrapless_path_iterator(
            map,
            continue_from.0,
            continue_from.1,
        ));
        if let Some((i, (_, _, _))) = path
            .iter()
            .enumerate()
            .find(|(i, (_, p, d))| *i != 0 && *p == pos && *d == dir)
        {
            path.truncate(i);
            return path.into_iter().cycle();
        }

        let (_, p, d) = path.last().unwrap();
        continue_from = cube_map(*p, *d);
    }
}
