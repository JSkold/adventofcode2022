use std::io::stdin;

const VIS_LEFT: u8 = 0b0001;
const VIS_RIGHT: u8 = 0b0010;
const VIS_UP: u8 = 0b0100;
const VIS_DOWN: u8 = 0b1000;

fn main() {
    let mut map: Vec<Vec<(u8, u8, u32)>> = Vec::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let chars = line.chars();
        map.push(
            chars
                .map(|c| (c.to_digit(10).unwrap() as u8, 0, 0))
                .collect(),
        );
    }

    // From left
    for r in map.iter_mut() {
        let mut h = 0;
        for t in r.iter_mut().enumerate() {
            if t.0 == 0 || (t.1).0 > h {
                (t.1).1 |= VIS_LEFT;
                h = (t.1).0;
            }
        }
    }

    // From right
    for r in map.iter_mut() {
        let mut h = 0;
        for t in r.iter_mut().rev().enumerate() {
            if t.0 == 0 || (t.1).0 > h {
                (t.1).1 |= VIS_RIGHT;
                h = (t.1).0;
            }
        }
    }

    // From up
    for c in 0..map[0].len() {
        let mut h = 0;
        for t in map
            .iter_mut()
            .map(|r| r.iter_mut().nth(c).unwrap())
            .enumerate()
            .collect::<Vec<_>>()
        {
            if t.0 == 0 || (t.1).0 > h {
                (t.1).1 |= VIS_UP;
                h = (t.1).0;
            }
        }
    }

    // From down
    for c in 0..map[0].len() {
        let mut h = 0;
        for t in map
            .iter_mut()
            .map(|r| r.iter_mut().nth(c).unwrap())
            .rev()
            .enumerate()
            .collect::<Vec<_>>()
        {
            if t.0 == 0 || (t.1).0 > h {
                (t.1).1 |= VIS_DOWN;
                h = (t.1).0;
            }
        }
    }

    // Part 2
    for x in 1..98 {
        for y in 1..98 {
            let h = map[y][x].0;
            let mut score = 1;

            // Horrible iterators, really hard to get them to do what I want.

            // Move left
            let mut l = map
                .iter()
                .nth(y)
                .unwrap()
                .iter()
                .rev()
                .skip(99 - x)
                .map_while(|p| if h > p.0 { Some(()) } else { None })
                .count()
                + 1;
            if l == x + 1 {
                l -= 1;
            }
            score *= l;

            // Move right
            let mut r = map
                .iter()
                .nth(y)
                .unwrap()
                .iter()
                .skip(x + 1)
                .map_while(|p| if h > p.0 { Some(()) } else { None })
                .count()
                + 1;
            if r == 99 - x {
                r -= 1;
            }
            score *= r;

            // Move up
            let mut u = map
                .iter()
                .rev()
                .skip(99 - y)
                .map_while(|p| if h > p[x].0 { Some(()) } else { None })
                .count()
                + 1;
            if u == y + 1 {
                u -= 1;
            }
            score *= u;

            // Move down
            let mut d = map
                .iter()
                .skip(y + 1)
                .map_while(|p| if h > p[x].0 { Some(()) } else { None })
                .count()
                + 1;
            if d == 99 - y {
                d -= 1;
            }
            score *= d;

            map[y][x].2 = score as u32;
        }
    }

    println!(
        "{}",
        map.iter()
            .map(|r| r.iter().filter(|t| t.1 > 0).count())
            .sum::<usize>()
    );

    println!(
        "{}",
        map.iter()
            .filter_map(|r| r.iter().map(|t| t.2).max())
            .max()
            .unwrap()
    );
}
