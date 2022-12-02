use std::io::stdin;

fn main() {
    let mut score_1 = 0;
    let mut score_2 = 0;
    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let op = match_input(line.chars().nth(0).unwrap());

        let char2 = line.chars().nth(2).unwrap();
        let me = match_input(char2);
        let res = match_input_res(char2);

        score_1 += me + (resolve(me, op) + 1) * 3;
        score_2 += rev_resolve(op, res) + (res + 1) * 3;
    }
    println!("{}", score_1);
    println!("{}", score_2);
}

fn match_input(input: char) -> i32 {
    match input {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(),
    }
}

fn match_input_res(input: char) -> i32 {
    match input {
        'X' => -1,
        'Y' => 0,
        'Z' => 1,
        _ => panic!(),
    }
}

fn resolve(a: i32, b: i32) -> i32 {
    match (a - b) % 3 {
        -2 => 1,
        2 => -1,
        i => i,
    }
}

fn rev_resolve(op: i32, res: i32) -> i32 {
    (op + res - 1).rem_euclid(3) + 1
}
