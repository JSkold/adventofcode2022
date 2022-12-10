use std::{
    io::{stdin, stdout, Write},
    ops::AddAssign,
};

fn sample_cycle(cycle: u32, reg_x: i32, sum_x: &mut i32, buffer: &mut String) {
    match cycle {
        20 => sum_x.add_assign(reg_x * 20),
        60 => sum_x.add_assign(reg_x * 60),
        100 => sum_x.add_assign(reg_x * 100),
        140 => sum_x.add_assign(reg_x * 140),
        180 => sum_x.add_assign(reg_x * 180),
        220 => sum_x.add_assign(reg_x * 220),
        _ => {}
    }

    if (((cycle as i32) % 40 - 1) - reg_x).abs() <= 1 {
        buffer.push('#');
    } else {
        buffer.push('.');
    }
}

fn main() {
    let mut cycle: u32 = 1;
    let mut reg_x = 1;

    let mut sum_x = 0;
    let mut buffer = String::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(' ');

        sample_cycle(cycle, reg_x, &mut sum_x, &mut buffer);

        match split.next().unwrap() {
            "addx" => {
                cycle += 1;
                sample_cycle(cycle, reg_x, &mut sum_x, &mut buffer);
                cycle += 1;
                reg_x += split.next().unwrap().parse::<i32>().unwrap();
            }
            "noop" => cycle += 1,
            _ => unimplemented!(),
        }
    }

    println!("{}", sum_x);
    let mut out = stdout().lock();
    out.write_all(buffer[0..40].as_bytes()).unwrap();
    println!();
    out.write_all(buffer[40..80].as_bytes()).unwrap();
    println!();
    out.write_all(buffer[80..120].as_bytes()).unwrap();
    println!();
    out.write_all(buffer[120..160].as_bytes()).unwrap();
    println!();
    out.write_all(buffer[160..200].as_bytes()).unwrap();
    println!();
    out.write_all(buffer[200..240].as_bytes()).unwrap();
    println!();
}
