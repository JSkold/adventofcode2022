use std::io::stdin;

fn main() {
    let mut sum = 0;

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        for (p, c) in line.chars().rev().enumerate() {
            match c {
                '1' => {
                    sum += 1 * 5_isize.pow(p as u32);
                }
                '2' => {
                    sum += 2 * 5_isize.pow(p as u32);
                }
                '0' => {}
                '-' => {
                    sum += -1 * 5_isize.pow(p as u32);
                }
                '=' => {
                    sum += -2 * 5_isize.pow(p as u32);
                }
                _ => unimplemented!(),
            }
        }
    }

    println!("{}", print_snafu(sum));
}

fn print_snafu(mut num: isize) -> String {
    let mut tmp = String::new();

    loop {
        let res = num % 5;
        match res {
            0 => tmp.push('0'),
            1 => {
                tmp.push('1');
                num -= 1;
            }
            2 => {
                tmp.push('2');
                num -= 2;
            }
            3 => {
                tmp.push('=');
                num += 2;
            }
            4 => {
                tmp.push('-');
                num += 1;
            }
            _ => unreachable!(),
        }
        num /= 5;
        if num == 0 {
            break;
        }
    }

    tmp.chars().rev().collect()
}
