use std::{fmt::Debug, io::stdin};

fn main() {
    let mut input_vec: Vec<_> = vec![];
    let mut placemnet_vec = vec![];

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        input_vec.push(line.parse::<isize>().unwrap());
    }
    for i in 0..input_vec.len() {
        placemnet_vec.push(i);
    }

    let mut output_vec = input_vec.clone();

    // i is the index 0..len
    // e is the element we are working with 1,2,-3,3,-2,0,4
    // p is the current position of e in the output vector
    for (i, e) in input_vec.iter().enumerate() {
        let p = placemnet_vec.iter().position(|j| *j == i).unwrap();

        shift(&mut output_vec, p, *e);
        shift(&mut placemnet_vec, p, *e)
    }

    let mut out_cycle = output_vec.iter().cycle();
    out_cycle.find(|n| **n == 0);

    // Task 1
    println!(
        "{}",
        out_cycle.nth(999).unwrap() + out_cycle.nth(999).unwrap() + out_cycle.nth(999).unwrap()
    );

    // ############################################################################################

    let mut placemnet_vec = vec![];
    for i in 0..input_vec.len() {
        placemnet_vec.push(i);
    }
    let input_vec: Vec<isize> = input_vec.into_iter().map(|n| n * 811589153).collect();
    let mut output_vec = input_vec.clone();

    for _ in 0..10 {
        for (i, e) in input_vec.iter().enumerate() {
            let p = placemnet_vec.iter().position(|j| *j == i).unwrap();

            shift(&mut output_vec, p, *e);
            shift(&mut placemnet_vec, p, *e)
        }
    }
    let mut out_cycle = output_vec.iter().cycle();
    out_cycle.find(|n| **n == 0);

    // Task 2
    println!(
        "{}",
        out_cycle.nth(999).unwrap() + out_cycle.nth(999).unwrap() + out_cycle.nth(999).unwrap()
    );
}

fn shift<T: Debug>(vec: &mut Vec<T>, mut index: usize, shift: isize) {
    let dir = shift.signum();

    // Won't keep the absolute positions as the example input, but since we are working with
    // cycles that doesn't matter.
    for _ in 0..(shift % (vec.len() as isize - 1)).abs() {
        let next_index = (index as isize + dir).rem_euclid(vec.len() as isize) as usize;
        vec.swap(index, next_index);
        index = next_index;
    }
}
