use std::{
    collections::VecDeque,
    io::{stdin, Lines, StdinLock},
};

fn main() {
    let mut lines = stdin().lines();
    let mut stacks = construct_stacks(&mut lines);
    let mut stacks2 = stacks.clone();

    // Skip empty line
    lines.next();
    let collected_lines: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();

    read_and_exec_ops(collected_lines.clone(), &mut stacks, false);
    read_and_exec_ops(collected_lines, &mut stacks2, true);

    println!("{}", get_top_string(&stacks));
    println!("{}", get_top_string(&stacks2));
}

fn construct_stacks(lines: &mut Lines<StdinLock>) -> Vec<VecDeque<char>> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    'outer: while let Some(Ok(line)) = lines.next() {
        let mut chars = line.chars();
        let mut pos = chars.nth(1);
        let mut i: usize = 0;
        while let Some(c) = pos {
            if stacks.len() as i32 - 1 < i as i32 {
                stacks.push(VecDeque::new());
            }
            if c.is_alphabetic() {
                stacks[i].push_back(c);
            } else if c.is_ascii_digit() {
                // We've reached the stack lables
                break 'outer;
            }
            pos = chars.nth(3);
            i += 1;
        }
    }
    stacks
}

fn read_and_exec_ops(lines: Vec<String>, stacks: &mut Vec<VecDeque<char>>, multi: bool) {
    let mut line_it = lines.into_iter();
    while let Some(line) = line_it.next() {
        let mut split = line.split_ascii_whitespace();
        let num = split.nth(1).unwrap().parse().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = split.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        if !multi {
            for _ in 0..num {
                let item = stacks[from].pop_front().unwrap();
                stacks[to].push_front(item);
            }
        } else {
            let mut tmp_queue = VecDeque::new();
            for _ in 0..num {
                tmp_queue.push_back(stacks[from].pop_front().unwrap());
            }
            for _ in 0..num {
                stacks[to].push_front(tmp_queue.pop_back().unwrap());
            }
        }
    }
}

fn get_top_string(stacks: &Vec<VecDeque<char>>) -> String {
    let mut output = String::new();
    let mut it = stacks.iter();
    while let Some(stack) = it.next() {
        if let Some(c) = stack.front() {
            output.push(*c);
        }
    }
    output
}
