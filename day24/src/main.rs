use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::stdin,
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut boundries: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut blizzards: Vec<((usize, usize), Direction)> = Vec::new();

    let mut max_x = 0;
    let mut max_y = 0;

    let mut lines = stdin().lines().enumerate().peekable();
    while let Some((y, Ok(line))) = lines.next() {
        for (x, c) in line.char_indices() {
            max_x = max(x, max_x);
            max_y = max(y, max_y);
            match c {
                '#' => {
                    boundries.insert((x, y), None);
                }
                '.' => {
                    if y == 0 {
                        start = (x, y);
                    } else if lines.peek().is_none() {
                        end = (x, y);
                    }
                }
                '<' => blizzards.push(((x, y), Direction::Left)),
                '>' => blizzards.push(((x, y), Direction::Right)),
                '^' => blizzards.push(((x, y), Direction::Up)),
                'v' => blizzards.push(((x, y), Direction::Down)),
                _ => unimplemented!(),
            }
        }
    }

    for y in 1..max_y {
        *boundries.get_mut(&(0, y)).unwrap() = Some((max_x - 1, y));
        *boundries.get_mut(&(max_x, y)).unwrap() = Some((1, y));
    }
    for x in 2..max_x - 1 {
        *boundries.get_mut(&(x, 0)).unwrap() = Some((x, max_y - 1));
        *boundries.get_mut(&(x, max_y)).unwrap() = Some((x, 1));
    }

    let mut score = path_find_breadth(start, end, &boundries, &mut blizzards);
    println!("{}", score);

    score += path_find_breadth(end, start, &boundries, &mut blizzards);
    score += path_find_breadth(start, end, &boundries, &mut blizzards);
    // Add 2 for each time we turn around.
    println!("{}", score + 2);
}

fn path_find_breadth(
    start: (usize, usize),
    end: (usize, usize),
    boundries: &HashMap<(usize, usize), Option<(usize, usize)>>,
    blizzards: &mut Vec<((usize, usize), Direction)>,
) -> usize {
    let d_it = vec![
        Direction::Down,
        Direction::Right,
        Direction::Left,
        Direction::Up,
    ];

    // Possible positions each iteration.
    let mut possible_position = HashSet::new();
    // Seed with starting position
    possible_position.insert(start);

    let mut it = 0;
    'outer: loop {
        let mut blizz_set = HashSet::new();

        // Update blizzard positions
        for (p, d) in blizzards.iter_mut() {
            let v_add = d.vector();

            p.0 = p.0.checked_add_signed(v_add.0).unwrap();
            p.1 = p.1.checked_add_signed(v_add.1).unwrap();

            // If the blizzard moved into a boundary
            if let Some(new_pos) = boundries.get(&p) {
                *p = new_pos.unwrap().clone();
            }
            blizz_set.insert(p.clone());
        }

        let pos: HashSet<_> = possible_position.drain().collect();
        for p in pos {
            if p == end {
                break 'outer;
            }

            for d in d_it.iter() {
                let v_add = d.vector();

                let new_pos = (
                    match p.0.checked_add_signed(v_add.0) {
                        Some(x) => x,
                        None => continue,
                    },
                    match p.1.checked_add_signed(v_add.1) {
                        Some(y) => y,
                        None => continue,
                    },
                );

                if !boundries.contains_key(&new_pos) && !blizz_set.contains(&new_pos) {
                    possible_position.insert(new_pos);
                }
                if !blizz_set.contains(&p) {
                    possible_position.insert(p);
                }
            }
        }

        it += 1;
    }

    it
}
