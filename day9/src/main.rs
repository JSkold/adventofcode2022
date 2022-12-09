use std::{collections::HashSet, io::stdin};

fn vector_add(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn vector_sub(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

fn inf_norm(a: &(isize, isize)) -> isize {
    std::cmp::max(a.0.abs(), a.1.abs())
}

fn comp_normalise(a: &(isize, isize)) -> (isize, isize) {
    (a.0 / a.0.abs(), a.1 / a.1.abs())
}

fn tail_step(diff: &(isize, isize)) -> (isize, isize) {
    if diff.0.abs() > 0 && diff.1.abs() > 0 {
        comp_normalise(diff)
    } else {
        let norm = inf_norm(&diff);
        ((diff.0 / norm), (diff.1 / norm))
    }
}

fn step_vector(dir: char, steps: usize) -> (isize, isize) {
    match dir {
        'R' => (steps as isize, 0),
        'L' => (-(steps as isize), 0),
        'U' => (0, steps as isize),
        'D' => (0, -(steps as isize)),
        _ => unimplemented!(),
    }
}

fn move_rope(
    rope: &mut Vec<(isize, isize)>,
    from_index: usize,
    visited: &mut HashSet<(isize, isize)>,
) {
    for i in from_index..rope.len() {
        while inf_norm(&vector_sub(&rope[i - 1], &rope[i])) > 1 {
            rope[i] = vector_add(&rope[i], &tail_step(&vector_sub(&rope[i - 1], &rope[i])));
            if i == rope.len() - 1 {
                visited.insert(rope[i]);
            }
            // If we move a rope section we have to make sure all subsequent sections are moved aswell
            move_rope(rope, i + 1, visited);
        }
    }
}

fn setup_rope(len: usize) -> (Vec<(isize, isize)>, HashSet<(isize, isize)>) {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut rope: Vec<(isize, isize)> = Vec::new();
    for _ in 0..len {
        rope.push((0, 0));
    }
    visited.insert(rope[len - 1]);
    (rope, visited)
}

fn main() {
    let (mut rope_short, mut visited_short) = setup_rope(2);
    let (mut rope_long, mut visited_long) = setup_rope(10);

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(' ');
        let dir = split.next().unwrap().parse::<char>().unwrap();
        let steps = split.next().unwrap().parse::<usize>().unwrap();

        rope_short[0] = vector_add(&rope_short[0], &step_vector(dir, steps));
        move_rope(&mut rope_short, 1, &mut visited_short);

        rope_long[0] = vector_add(&rope_long[0], &step_vector(dir, steps));
        move_rope(&mut rope_long, 1, &mut visited_long);
    }

    println!("{}", visited_short.len());
    println!("{}", visited_long.len());
}
