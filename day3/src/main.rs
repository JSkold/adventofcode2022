use std::{
    collections::{hash_map::RandomState, HashSet},
    io::stdin,
};

fn main() {
    let mut lines = stdin().lines();

    let mut tot: u64 = 0;
    let mut all: Vec<HashSet<u8>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let mut set = HashSet::new();
        let bytes: Vec<u8> = line.bytes().collect();
        let mut items: Vec<u8> = Vec::new();

        for byte in bytes {
            let item = get_val(byte);
            set.insert(item);
            items.push(item);
        }

        let half2 = HashSet::<_, RandomState>::from_iter(items.split_off(items.len() / 2));
        let half1 = HashSet::<_, RandomState>::from_iter(items);

        tot += *half1.intersection(&half2).next().unwrap() as u64;
        all.push(set);
    }

    let mut grp_tot: u64 = 0;
    let mut group = all;

    while group.len() > 0 {
        let rest = group.split_off(3);
        let (i, other) = group.split_at_mut(1);
        let i = &mut i[0];
        for j in other {
            i.retain(|k| j.contains(k));
        }
        let val = *i.iter().next().unwrap() as u64;
        grp_tot += val;
        group = rest;
    }

    println!("{}", tot);
    println!("{}", grp_tot);
}

fn get_val(char: u8) -> u8 {
    match char {
        65..=90 => char - 64 + 26,
        97..=123 => char - 96,
        _ => panic!(),
    }
}
