use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let mut any_overlap: u32 = 0;
    let mut complete_overlap: u32 = 0;

    while let Some(Ok(line)) = lines.next() {
        let sections_str: Vec<_> = line.split(',').collect();
        let sec1 = parse_section(sections_str[0]);
        let sec2 = parse_section(sections_str[1]);

        if sec1.0 <= sec2.1 && sec1.1 >= sec2.0 {
            any_overlap += 1;
        }
        if (sec1.1 <= sec2.1 && sec1.0 >= sec2.0) || (sec2.1 <= sec1.1 && sec2.0 >= sec1.0) {
            complete_overlap += 1;
        }
    }
    println!("{}", complete_overlap);
    println!("{}", any_overlap);
}

fn parse_section(section_str: &str) -> (u32, u32) {
    let idxs: Vec<_> = section_str.split('-').collect();
    (
        idxs[0].parse::<u32>().unwrap(),
        idxs[1].parse::<u32>().unwrap(),
    )
}
