use std::{
    cmp::{max, min},
    collections::HashSet,
    io::stdin,
};

fn main() {
    let mut set: HashSet<(isize, isize, isize)> = HashSet::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<isize>().unwrap();
        let y = split.next().unwrap().parse::<isize>().unwrap();
        let z = split.next().unwrap().parse::<isize>().unwrap();

        set.insert((x, y, z));
    }

    // Create all adjacent positions
    let adj = vec![
        (1, 0, 0),
        (0, 1, 0),
        (0, 0, 1),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 0, -1),
    ];

    // Grab some variables we need for part 2
    let mut x_bounds = (isize::MAX, isize::MIN);
    let mut y_bounds = (isize::MAX, isize::MIN);
    let mut z_bounds = (isize::MAX, isize::MIN);

    let mut sides = 0;
    for (x, y, z) in set.iter() {
        for (ax, ay, az) in adj.iter() {
            // Any adjacent block not in set is a boundary
            if !set.contains(&(x + ax, y + ay, z + az)) {
                sides += 1;
            }
        }

        // Get the sides of a cube covering the entire space
        x_bounds.0 = min(x_bounds.0, *x);
        x_bounds.1 = max(x_bounds.1, *x);
        y_bounds.0 = min(y_bounds.0, *y);
        y_bounds.1 = max(y_bounds.1, *y);
        z_bounds.0 = min(z_bounds.0, *z);
        z_bounds.1 = max(z_bounds.1, *z);
    }

    // Will hold all sets of blocks that are reachable from the outside
    let mut outside: HashSet<(isize, isize, isize)> = HashSet::new();
    // Insert initial value outside the bounds of space
    outside.insert((x_bounds.0 - 1, y_bounds.0 - 1, z_bounds.0 - 1));

    loop {
        let mut new: HashSet<(isize, isize, isize)> = HashSet::new();
        let mut out_it = outside.iter();
        while let Some((x, y, z)) = out_it.next() {
            let steps: Vec<(isize, isize, isize)> = adj
                .iter()
                .map(|(ax, ay, az)| (*x + *ax, *y + *ay, *z + *az))
                .filter(|(px, py, pz)| {
                    // We must be within boundary plus one additional block,
                    // extra block guarantees we can visit all blocks.
                    *px >= x_bounds.0 - 1
                        && *px <= x_bounds.1 + 1
                        && *py >= y_bounds.0 - 1
                        && *py <= y_bounds.1 + 1
                        && *pz >= z_bounds.0 - 1
                        && *pz <= z_bounds.1 + 1
                })
                .filter(|p| !outside.contains(p))
                .filter(|p| !set.contains(p))
                .filter(|p| !new.contains(p))
                .collect();

            for step in steps {
                new.insert(step);
            }
        }

        if new.len() == 0 {
            break;
        } else {
            new.drain().for_each(|p| {
                outside.insert(p);
            })
        }
    }

    let mut external_sides = 0;
    for (x, y, z) in set.iter() {
        for (ax, ay, az) in adj.iter() {
            // Any block adjacent to an outside block will have an external side.
            if outside.contains(&(x + ax, y + ay, z + az)) {
                external_sides += 1;
            }
        }
    }

    println!("{}", sides);
    println!("{}", external_sides);
}
