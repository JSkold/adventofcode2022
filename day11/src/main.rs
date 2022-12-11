use std::{cell::RefCell, collections::VecDeque, io::stdin, rc::Rc};

#[derive(Clone)]
struct Monkey {
    items: VecDeque<i64>,
    op: Rc<dyn Fn(i64) -> i64>,
    throw: Option<Rc<dyn Fn(i64)>>,
}

fn setup_monkey_throw(
    monkeys: &mut Vec<Rc<RefCell<Monkey>>>,
    monkey_refs: &Vec<(i32, usize, usize)>,
) {
    assert!(monkeys.len() == monkey_refs.len());
    for i in 0..monkeys.len() {
        let div_by = monkey_refs[i].0;
        let if_true = monkeys[monkey_refs[i].1].clone();
        let if_false = monkeys[monkey_refs[i].2].clone();

        // Setup throw closure, takes a worry value representing an item, push to correct monkey
        let throw: Rc<dyn Fn(i64)> = Rc::new(move |i| {
            if i % div_by as i64 == 0 {
                if_true.borrow_mut().items.push_back(i);
            } else {
                if_false.borrow_mut().items.push_back(i);
            }
        });

        // Assign Monkey.throw, should be no None now
        monkeys[i].borrow_mut().throw = Some(throw);
    }
}

fn main() {
    // Monkeys need to be able to refer to each other => Rc
    // Monkeys need to be able to push items to each other => RefCell
    // Provided monkeys don't throw to themselves, this is safe.
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    // We need to create all monkeys before assigning who they throw to, save who they throw to
    // in this vector. .0 test divisible by, .1 to monkey if test is true, .2 if false.
    let mut monkey_refs: Vec<(i32, usize, usize)> = Vec::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("Monkey") {
            // Starting items:
            let items = lines
                .next()
                .unwrap()
                .unwrap()
                .split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|str| str.trim().parse::<i64>().unwrap())
                .collect();

            // Operation:
            let op_split: Vec<String> = lines
                .next()
                .unwrap()
                .unwrap()
                .split('=')
                .nth(1)
                .unwrap()
                .trim()
                .split(' ')
                .map(String::from)
                .collect();
            assert!(op_split[0] == "old");
            let op: Rc<dyn Fn(i64) -> i64> = match op_split[2].as_str() {
                "old" => match op_split[1].as_str() {
                    "+" => Rc::new(move |a| a * 2),
                    "*" => Rc::new(move |a| a.pow(2)),
                    _ => unimplemented!(),
                },
                _ => {
                    let math_const = op_split[2].parse::<i64>().unwrap();
                    match op_split[1].as_str() {
                        "+" => Rc::new(move |a| a + math_const),
                        "*" => Rc::new(move |a| a * math_const),
                        _ => unimplemented!(),
                    }
                }
            };

            // Test:
            // Divisible by is the only test operation
            let div_by = lines
                .next()
                .unwrap()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let monkey_test_true = lines
                .next()
                .unwrap()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let monkey_test_false = lines
                .next()
                .unwrap()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            // Push monkey references to separate vector. Assign throw later.
            monkey_refs.push((div_by, monkey_test_true, monkey_test_false));
            monkeys.push(Rc::new(RefCell::new(Monkey {
                items,
                op,
                throw: None,
            })));
        }
    }

    // Deep clone monkeys for second challange.
    let mut monkeys2: Vec<Rc<RefCell<Monkey>>> = monkeys
        .iter()
        .map(|m| Rc::new(RefCell::new((*m.borrow()).clone())))
        .collect();

    // Setup throw closure between monkeys.
    setup_monkey_throw(&mut monkeys, &monkey_refs);
    setup_monkey_throw(&mut monkeys2, &monkey_refs);

    // Calculate a common factor between the divisable value of all monkeys.
    // If we stop the worry value from overflowing this value we can prevent it from
    // growing indefinately while keeping the monkey calculations unchanged.
    let common_factor: i64 = monkey_refs.iter().map(|m| m.0 as i64).product();

    let mut inspect: Vec<u64> = vec![0; monkeys.len()];
    let mut inspect2: Vec<u64> = vec![0; monkeys2.len()];

    // Loop rounds
    for r in 0..10000 {
        // First challange: Loop turns (only up to 20)
        if r < 20 {
            for (i, m) in monkeys.iter_mut().enumerate() {
                // Loop items
                let mut monkey = m.borrow_mut();
                while let Some(item) = monkey.items.pop_front() {
                    inspect[i] += 1;
                    monkey.throw.as_ref().unwrap()(monkey.op.as_ref()(item) / 3);
                }
            }
        }

        // Second challange: Loop turns
        for (i, m) in monkeys2.iter_mut().enumerate() {
            // Loop items
            let mut monkey = m.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                inspect2[i] += 1;
                monkey.throw.as_ref().unwrap()(monkey.op.as_ref()(item).rem_euclid(common_factor));
            }
        }
    }

    inspect.sort();
    inspect2.sort();

    println!(
        "{}",
        inspect
            .rchunks(2)
            .map(|c| c.iter().product::<u64>())
            .next()
            .unwrap()
    );
    println!(
        "{}",
        inspect2
            .rchunks(2)
            .map(|c| c.iter().product::<u64>())
            .next()
            .unwrap()
    );
}
