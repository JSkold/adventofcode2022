use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

fn main() {
    let mut lines = stdin().lines();

    while let Some(Ok(line)) = lines.next() {
        println!("{}", process_signal(line.clone(), 4).unwrap());
        println!("{}", process_signal(line.clone(), 14).unwrap());
    }
}

fn process_signal(line: String, num_distinct: usize) -> Option<usize> {
    let mut it = line.char_indices();
    let mut queue: VecDeque<char> = VecDeque::new();
    let mut map: HashMap<char, usize> = HashMap::new();

    while let Some((i, char)) = it.next() {
        if let Some(num) = map.get_mut(&char) {
            *num += 1;
        } else {
            map.insert(char, 1);
        }
        queue.push_front(char);

        if queue.len() > num_distinct {
            let rem = queue.pop_back().unwrap();
            let num = map.get_mut(&rem).unwrap();
            if *num == 1 {
                map.remove(&rem);
            } else {
                *num -= 1;
            }
            if map.len() == num_distinct {
                return Some(i + 1);
            }
        }
    }
    return None;
}
