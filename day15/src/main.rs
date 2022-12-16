use std::{
    cmp::{max, min},
    collections::HashSet,
    io::stdin,
    ops::Range,
};

struct Sensor {
    pos: (isize, isize),
    dist: isize,
}

impl Sensor {
    fn new(s_pos: (isize, isize), b_pos: (isize, isize)) -> Self {
        Sensor {
            pos: s_pos,
            dist: manhattan_dist(&s_pos, &b_pos),
        }
    }

    fn coverage_at_row(&self, r: isize) -> SensorRange {
        let diff = (self.pos.1 - r).abs();
        if diff <= self.dist {
            SensorRange {
                r: ((self.pos.0 - (self.dist - diff))..(self.pos.0 + (self.dist - diff) + 1)),
            }
        } else {
            SensorRange { r: 0..0 }
        }
    }
}

#[derive(Eq, PartialEq)]
struct SensorRange {
    r: Range<isize>,
}

impl SensorRange {
    fn start(&self) -> isize {
        self.r.start
    }
    fn end(&self) -> isize {
        self.r.end
    }
}

impl PartialOrd for SensorRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SensorRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Order on start of range first, if equal, then on end.
        match self.r.start.cmp(&other.r.start) {
            std::cmp::Ordering::Equal => self.r.end.cmp(&other.r.end),
            o => o,
        }
    }
}

fn manhattan_dist(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// Collapse a vec of sorted SensorRanges into the smallest possible set.
/// No ranges will overlap after this operation.
fn collapse_ranges(ranges: &mut Vec<SensorRange>) {
    'outer: loop {
        for i in 0..ranges.len() - 1 {
            let cur = &ranges[i];
            let next = &ranges[i + 1];

            if cur.start() == next.start() {
                ranges.remove(i);
                continue 'outer;
            }
            if next.start() <= cur.end() {
                let replacement_range = SensorRange {
                    r: cur.start()..max(cur.end(), next.end()),
                };
                ranges[i] = replacement_range;
                ranges.remove(i + 1);
                continue 'outer;
            }
        }
        break;
    }
}

fn clamp_ranges(ranges: &mut Vec<SensorRange>, clamp: Range<isize>) {
    ranges
        .iter_mut()
        .for_each(|r| r.r = max(r.start(), clamp.start)..min(r.end(), clamp.end));
    collapse_ranges(ranges);
}

fn row_coverage(sensors: &Vec<Sensor>, y: isize) -> Vec<SensorRange> {
    let mut row_coverage: Vec<SensorRange> = Vec::new();
    for s in sensors {
        let sensor_range = s.coverage_at_row(y);
        row_coverage.insert(
            row_coverage
                .binary_search(&sensor_range)
                .unwrap_or_else(|e| e),
            sensor_range,
        );
    }
    collapse_ranges(&mut row_coverage);
    row_coverage
}

fn main() {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut known_beacons: HashSet<(isize, isize)> = HashSet::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(['=', ',', ':']);
        let s = (
            split.nth(1).unwrap().parse::<isize>().unwrap(),
            split.nth(1).unwrap().parse::<isize>().unwrap(),
        );
        let b = (
            split.nth(1).unwrap().parse::<isize>().unwrap(),
            split.nth(1).unwrap().parse::<isize>().unwrap(),
        );

        sensors.push(Sensor::new(s, b));
        known_beacons.insert(b);
    }

    // Get the ranges of coverage on row 2000000
    let row = row_coverage(&sensors, 2_000_000);
    // Find the number of beacons on the row that are also in any of the ranges. These must be subtracted.
    let beacons_in_row = known_beacons
        .iter()
        .filter_map(|b| if b.1 == 2_000_000 { Some(b.0) } else { None })
        .filter(|x| row.iter().any(|r| r.r.contains(x)))
        .count();
    // Sum the coverage from all the ranges.
    let coverage_count: isize = row.iter().map(|r| r.end() - r.start()).sum::<isize>();
    println!("{}", coverage_count - beacons_in_row as isize);

    // The proper way to solve task 2 would be to generalise the range method to 2D but since it's
    // resonably quick to calculate the ranges for 1 row we just repeat it 4 million times.
    for y in 0..=4_000_000 {
        let mut row: Vec<SensorRange> = row_coverage(&sensors, y);
        clamp_ranges(&mut row, 0..4_000_000);
        // Since there is only a single position not covered in the domain we are searching we
        // simply look for the first row that has disjoint coverage range. This assumes that the
        // position we are looking for is not on the edge.
        if row.len() > 1 {
            println!("{}", row[0].end() * 4_000_000 + y);
            break;
        }
    }
}
