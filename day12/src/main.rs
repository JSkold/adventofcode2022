use std::{collections::HashMap, io::stdin};

fn vector_add(a: &(usize, usize), b: &(isize, isize)) -> Option<(usize, usize)> {
    let c = (a.0.checked_add_signed(b.0), a.1.checked_add_signed(b.1));
    if c.0.is_none() || c.1.is_none() {
        None
    } else {
        Some((c.0.unwrap(), c.1.unwrap()))
    }
}

fn path_find(
    start: &(usize, usize),
    end: &(usize, usize),
    map: &Vec<Vec<u8>>,
) -> Option<Vec<(usize, usize)>> {
    let mut path_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let total_map_size: usize = map.iter().map(|row| row.len()).sum();
    path_map.insert(start.clone(), Vec::new());

    while path_map.len() < total_map_size {
        let mut new_steps: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        let mut path_it = path_map.iter();
        while let Some((pos, path)) = path_it.next() {
            let possible_steps = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            let steps: Vec<(usize, usize)> = possible_steps
                .iter()
                .filter_map(|p| vector_add(pos, p))
                .filter(|p| p.1 < map.len() && p.0 < map[p.1].len())
                .filter(|p| !path_map.contains_key(p))
                .filter(|p| !new_steps.contains_key(p))
                .filter(|p| map[p.1][p.0] as i8 - map[pos.1][pos.0] as i8 <= 1)
                .collect();

            for step in steps {
                let mut new_path = path.clone();
                new_path.push(step.clone());
                new_steps.insert(step.clone(), new_path);
            }
        }
        // If no new steps were found, then break early.
        if new_steps.len() == 0 {
            break;
        }
        new_steps
            .drain()
            .for_each(|(k, v)| assert!(path_map.insert(k, v).is_none()));
    }

    path_map.get(&end).cloned()
}

fn main() {
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        map.push(
            line.bytes()
                .enumerate()
                .map(|(x, b)| match b {
                    b'S' => {
                        start_pos = (x, map.len());
                        0
                    }
                    b'E' => {
                        end_pos = (x, map.len());
                        b'z' - 97
                    }
                    b => b - 97,
                })
                .collect(),
        );
    }

    let start_positions: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, v)| if *v == 0 { Some(x) } else { None })
                .map(move |x| (x, y))
        })
        .flatten()
        .collect();

    println!("{}", path_find(&start_pos, &end_pos, &map).unwrap().len());
    println!(
        "{}",
        start_positions
            .iter()
            .filter_map(|s| path_find(&s, &end_pos, &map))
            .map(|p| p.len())
            .min()
            .unwrap()
    );
}
