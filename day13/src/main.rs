use std::{cmp::Ordering, io::stdin};

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Num(u8),
    Arr(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Num(n1) => match other {
                Packet::Num(n2) => n1.cmp(&n2),
                Packet::Arr(_a2) => Packet::Arr(vec![Packet::Num(*n1)]).cmp(&other),
            },
            Packet::Arr(a1) => match other {
                Packet::Num(n2) => self.cmp(&Packet::Arr(vec![Packet::Num(*n2)])),
                Packet::Arr(a2) => a1
                    .iter()
                    .zip(a2.iter())
                    .map(|(a, b)| a.cmp(b))
                    .find(|r| *r != Ordering::Equal)
                    .unwrap_or(a1.len().cmp(&a2.len())),
            },
        }
    }
}

fn parse_line(line: &str) -> (Packet, usize) {
    assert!(line.chars().next() == Some('['));

    // This function will always return a Packet::Arr
    let mut packet_arr: Vec<Packet> = Vec::new();

    // Keep track of how far we've parsed the current line
    let mut parse_length = 1;
    let mut sub_section = &line[1..];
    loop {
        // Find the next point we either recurse or return
        let stop = sub_section.find(['[', ']']).unwrap();
        parse_length += stop;

        // Parse all numbers before stop point
        let _ = &sub_section[..stop]
            .split(',')
            .filter(|s| s.len() > 0)
            .for_each(|n| {
                packet_arr.push(Packet::Num(
                    n.trim_matches(&['[', ']'] as &[char])
                        .parse::<u8>()
                        .unwrap(),
                ));
            });

        // Determine what happens at spot point
        // '[' -> recurse
        // ']' -> return
        let stop_char = *(&sub_section.chars().nth(stop));
        if stop_char == Some(']') {
            // + 1 on parse_length to account for ']' character
            // '[' is already accouned for due to initial value
            return (Packet::Arr(packet_arr), parse_length + 1);
        } else if stop_char == Some('[') {
            let (nested_packet, steps) = parse_line(&sub_section[stop..]);
            packet_arr.push(nested_packet);
            // Make sure we skip part of line that was nested packets
            parse_length += steps;
            sub_section = &sub_section[stop + steps..];
        } else {
            // We must have a stop point
            panic!()
        }
    }
}

fn main() {
    let lines: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();

    // Sum index+1 of all ordered pairs
    let num = lines
        .chunks(3)
        .enumerate()
        .filter_map(|(i, l)| {
            // Third part of chunk should be empty line
            let (p1, _) = parse_line(&l[0]);
            let (p2, _) = parse_line(&l[1]);
            if p1 < p2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    // Collect all packets into vector
    let mut packets: Vec<Packet> = lines
        .iter()
        .filter(|s| s.len() > 0)
        .filter_map(|s| Some(parse_line(&s)))
        .map(|(p, _)| p)
        .collect();

    // Insert market packets
    let marker1 = Packet::Arr(vec![Packet::Arr(vec![Packet::Num(2)])]);
    let marker2 = Packet::Arr(vec![Packet::Arr(vec![Packet::Num(6)])]);
    packets.push(marker1.clone());
    packets.push(marker2.clone());
    packets.sort();

    println!("{}", num);
    println!(
        "{}",
        // Binary search since packets is now sorted
        (packets.binary_search(&marker1).unwrap() + 1)
            * (packets.binary_search(&marker2).unwrap() + 1)
    );
}
