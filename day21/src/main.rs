use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn inverse(self) -> Self {
        match self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Mul => Operation::Div,
            Operation::Div => Operation::Mul,
        }
    }
}

fn main() {
    let mut num_monkeys = HashMap::new();
    let mut op_monkeys = HashMap::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(':');
        let name = split.next().unwrap().to_string();

        let monkey_val = split.next().unwrap();
        let mut op_split = monkey_val.split(['+', '-', '*', '/']);
        if let (Some(term1), Some(term2)) = (op_split.next(), op_split.next()) {
            let term1 = term1.trim().to_string();
            let term2 = term2.trim().to_string();

            let op = if monkey_val.find("+").is_some() {
                Operation::Add
            } else if monkey_val.find("-").is_some() {
                Operation::Sub
            } else if monkey_val.find("*").is_some() {
                Operation::Mul
            } else if monkey_val.find("/").is_some() {
                Operation::Div
            } else {
                unimplemented!()
            };

            op_monkeys.insert(name, (term1, term2, op));
        } else {
            let num = monkey_val.trim().parse::<isize>().unwrap();
            num_monkeys.insert(name, num);
        }
    }

    // Task 1
    println!(
        "{}",
        recursive_complete(&num_monkeys, &op_monkeys, &"root".to_string())
    );

    // Remove the unused "humn" value.
    num_monkeys.remove(&"humn".to_string());
    // Change the "root" equality into a subtraction.
    op_monkeys.get_mut(&"root".to_string()).unwrap().2 = Operation::Sub;
    // LHS = RHS  <=>  LHS - RHS = 0
    // Insert "root" as zero
    num_monkeys.insert("root".to_string(), 0);

    // Recursively rearrange equation using "humn" to be LHS.
    recursive_rearrange(&num_monkeys, &mut op_monkeys, &"humn".to_string());
    // Solve for "humn" the same way solved for "root" in task 1
    println!(
        "{}",
        recursive_complete(&num_monkeys, &op_monkeys, &"humn".to_string())
    );
}

fn recursive_complete(
    num_monkeys: &HashMap<String, isize>,
    op_monkeys: &HashMap<String, (String, String, Operation)>,
    needle: &String,
) -> isize {
    // Try to find value in numbers map first, fallback to operation and solve recursively.
    match num_monkeys.get(needle) {
        Some(n) => *n,
        None => {
            let op = op_monkeys.get(needle).unwrap();
            let term1 = recursive_complete(num_monkeys, op_monkeys, &op.0);
            let term2 = recursive_complete(num_monkeys, op_monkeys, &op.1);
            match op.2 {
                Operation::Add => term1 + term2,
                Operation::Sub => term1 - term2,
                Operation::Mul => term1 * term2,
                Operation::Div => term1 / term2,
            }
        }
    }
}

fn recursive_rearrange(
    num_monkeys: &HashMap<String, isize>,
    op_monkeys: &mut HashMap<String, (String, String, Operation)>,
    needle: &String,
) {
    // Find the LHS of equation containing needle
    if let Some(target_key) = op_monkeys.iter().find_map(|(m, op)| {
        if op.0 == *needle || op.1 == *needle {
            Some(m.clone())
        } else {
            None
        }
    }) {
        // Pop the target equation, recursively rearrange any other equation using the target key as a term.
        let target = op_monkeys.remove(&target_key).unwrap();
        recursive_rearrange(num_monkeys, op_monkeys, &target_key);
        // Now reinsert the rearranged equation.
        match target.0 == *needle {
            true => {
                // If needle is the positive value in the subtraction we can just change the
                // operation to addition.
                op_monkeys.insert(needle.clone(), (target_key, target.1, target.2.inverse()));
            }
            false => match target.2 {
                // Since the needle is the negative term in this case we have to rearrange the equation
                // into another subtraction operation.
                Operation::Sub => {
                    op_monkeys.insert(needle.clone(), (target.0, target_key, Operation::Sub));
                }
                _ => {
                    op_monkeys.insert(needle.clone(), (target_key, target.0, target.2.inverse()));
                }
            },
        }
    }
}
