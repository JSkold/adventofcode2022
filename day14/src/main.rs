#![feature(is_some_and)]
#![feature(iter_next_chunk)]

use std::{cmp::max, io::stdin};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<Material>>,
    low_bound: Option<(isize, isize)>,
    high_bound: Option<(isize, isize)>,
}

impl Map {
    fn new() -> Map {
        Map {
            map: Vec::new(),
            low_bound: None,
            high_bound: None,
        }
    }

    fn material_count(&self, mat: Material) -> usize {
        self.map
            .iter()
            .map(|r| r.iter().filter(|m| **m == mat).count())
            .sum()
    }

    fn bounds(&self) -> ((isize, isize), (isize, isize)) {
        (self.low_bound.unwrap(), self.high_bound.unwrap())
    }

    fn bounds_check(&mut self, pos: &(isize, isize)) {
        if self.low_bound.is_none() {
            self.low_bound = Some(pos.clone())
        }
        if self.high_bound.is_none() {
            self.high_bound = Some(pos.clone())
        }

        self.expand_map_low_bounds(&vector_neg(&vector_sub(&pos, &self.low_bound.unwrap())));
        self.expand_map_high_bounds(&vector_sub(
            &vector_add(&pos, &(1, 1)),
            &self.high_bound.unwrap(),
        ));
    }

    fn get_material(&self, pos: &(isize, isize)) -> Option<Material> {
        if let (Some(low), Some(high)) = (self.low_bound, self.high_bound) {
            if pos.0 < low.0 || pos.1 < low.1 || pos.0 >= high.0 || pos.1 >= high.1 {
                None
            } else {
                Some(
                    self.map[(pos.1 - self.low_bound.unwrap().1) as usize]
                        [(pos.0 - self.low_bound.unwrap().0) as usize],
                )
            }
        } else {
            None
        }
    }

    fn set_material(&mut self, pos: &(isize, isize), mat: Material) {
        self.bounds_check(pos);
        self.map[(pos.1 - self.low_bound.unwrap().1) as usize]
            [(pos.0 - self.low_bound.unwrap().0) as usize] = mat;
    }

    fn set_material_line(&mut self, start: &(isize, isize), end: &(isize, isize), mat: Material) {
        let line = vector_sub(&end, &start);
        // We only allow lines parallel to the basis vectors
        assert!(line.0 == 0 || line.1 == 0);

        let step = vector_normalise(&line);
        let mut p = *start;
        while p != *end {
            self.set_material(&p, mat);
            p = vector_add(&p, &step);
        }
        self.set_material(&p, mat);
    }

    fn expand_map_low_bounds(&mut self, vec: &(isize, isize)) {
        let (x, y) = *vec;
        if y > 0 {
            let len = self.map.get(0).map_or(0, |r| r.len());
            self.map
                .splice(0..0, vec![vec![Material::Air; len]; y as usize]);
            self.low_bound.unwrap().1 -= y;
        }
        if x > 0 {
            self.map.iter_mut().for_each(|r| {
                r.splice(0..0, vec![Material::Air; x as usize]);
            });
        }
        self.low_bound = Some(vector_sub(
            &self.low_bound.unwrap(),
            &vector_clamp_pos(&vec),
        ));
    }

    fn expand_map_high_bounds(&mut self, vec: &(isize, isize)) {
        let (x, y) = *vec;
        if y > 0 {
            let len = self.map.get(0).map_or(0, |r| r.len());
            self.map
                .append(&mut vec![vec![Material::Air; len]; y as usize]);
        }
        if x > 0 {
            self.map
                .iter_mut()
                .for_each(|r| r.append(&mut vec![Material::Air; x as usize]));
        }
        self.high_bound = Some(vector_add(
            &self.high_bound.unwrap(),
            &vector_clamp_pos(&vec),
        ));
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map
            .iter()
            .map(|r| {
                f.write_str(
                    &(r.iter()
                        .map(|m| match m {
                            Material::Air => '.',
                            Material::Rock => '#',
                            Material::Sand => '+',
                        })
                        .collect::<String>()
                        + "\n"),
                )
            })
            .find(Result::is_err)
            .unwrap_or(Ok(()))
    }
}

fn vector_add(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn vector_sub(a: &(isize, isize), b: &(isize, isize)) -> (isize, isize) {
    (a.0 - b.0, a.1 - b.1)
}

fn vector_clamp_pos(a: &(isize, isize)) -> (isize, isize) {
    (a.0.clamp(0, isize::MAX), a.1.clamp(0, isize::MAX))
}

fn vector_neg(a: &(isize, isize)) -> (isize, isize) {
    (-a.0, -a.1)
}

fn vector_normalise(a: &(isize, isize)) -> (isize, isize) {
    // Since we only work with vectors parallel to one of the basis vectors
    let length = max(a.0.abs(), a.1.abs());
    (a.0 / length, a.1 / length)
}

fn create_map() -> Map {
    let mut map: Map = Map::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let vertices: Vec<(isize, isize)> = line
            .split(" -> ")
            .map(|s| {
                s.split(',')
                    .next_chunk()
                    .map(|[x_str, y_str]| {
                        (
                            x_str.parse::<isize>().unwrap(),
                            y_str.parse::<isize>().unwrap(),
                        )
                    })
                    .unwrap()
            })
            .collect(); // Collect makes it easier to work with i-1

        for i in 1..vertices.len() {
            let prev = vertices[i - 1];
            let cur = vertices[i];
            map.set_material_line(&prev, &cur, Material::Rock);
        }
    }

    map
}

fn simulate_sand(map: &mut Map) {
    // Possible positions for the sand to fall into.
    let candidate_positions = [(0, 1), (-1, 1), (1, 1)];

    'outer: loop {
        let mut prev_pos = None;
        let mut sand_pos = (500, 0);
        loop {
            if let Some(p) = prev_pos {
                let next = candidate_positions
                    .iter()
                    .map(|c| (vector_add(&p, c), map.get_material(&vector_add(&p, c))))
                    .filter(|(_, m)| m.is_none() || m.is_some_and(|rm| rm == Material::Air))
                    .next();
                match next {
                    Some((next_pos, mat)) => {
                        // We set the prev position to air in either case, the sand
                        // either flows outside map, or moved to next position.
                        map.set_material(&p, Material::Air);
                        match mat {
                            Some(_) => {
                                sand_pos = next_pos;
                            }
                            None => break 'outer, // Falling utside map
                        }
                    }
                    None => break, // No more moves
                }
            } else if let Some(m) = map.get_material(&sand_pos) {
                if m == Material::Sand {
                    // If we have sand at the starting position
                    break 'outer;
                }
            }

            map.set_material(&sand_pos, Material::Sand);
            prev_pos = Some(sand_pos);
        }
    }
}

fn main() {
    let mut map = create_map();
    let mut map2 = map.clone();

    simulate_sand(&mut map);

    let (_low, high) = map2.bounds();
    let floor_level = high.1 + 1;
    // The maximum width of the sand pile can be 2h+1, where h is the height.
    // We add an extra +1/-1 to make sure we break the simulation loop when the
    // start position is occupied and not when any sand spills outside.
    map2.set_material_line(
        &(500 - floor_level - 1, floor_level),
        &(500 + floor_level + 1, floor_level),
        Material::Rock,
    );

    simulate_sand(&mut map2);

    println!("{}", map.material_count(Material::Sand));
    println!("{}", map2.material_count(Material::Sand));
}
