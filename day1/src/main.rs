use std::{io::stdin, ops::AddAssign};

fn main() {
    let mut elves: Vec<u32> = Vec::new();
    elves.push(0);
    let mut cur_elf = elves.last_mut().unwrap();

    let stdin = stdin();
    for line in stdin.lines() {
        if line.as_ref().expect("Read error").len() == 0 {
            elves.push(0);
            cur_elf = elves.last_mut().unwrap();
            continue;
        }

        cur_elf.add_assign(line.unwrap().parse::<u32>().unwrap());
    }

    elves.sort();
    elves.reverse();
    println!("{}", elves[0]);
    println!("{}", elves[0] + elves[1] + elves[2]);
}
