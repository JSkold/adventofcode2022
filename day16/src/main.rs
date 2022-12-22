use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::stdin,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(Clone)]
struct Node {
    flow: usize,
    // Edges between nodes
    connections: HashSet<usize>,
    // Cost to travel to any node in network
    costs: HashMap<usize, usize>,
}

fn main() {
    let mut nodes: HashMap<usize, Node> = HashMap::new();
    let mut name_map: HashMap<String, usize> = HashMap::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.splitn(10, ' ');
        let name = split.nth(1).unwrap().to_string();
        let flow = split
            .nth(2)
            .unwrap()
            .trim_start_matches("rate=")
            .trim_end_matches(';')
            .parse::<usize>()
            .unwrap();
        let connections: HashSet<String> = split
            .nth(4)
            .unwrap()
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

        if !name_map.contains_key(&name) {
            name_map.insert(name.clone(), name_map.len());
        }
        connections.iter().for_each(|n| {
            if !name_map.contains_key(n) {
                name_map.insert(n.clone(), name_map.len());
            }
        });
        let connections = connections
            .into_iter()
            .map(|s| *name_map.get(&s).unwrap())
            .collect();

        nodes.insert(
            *name_map.get(&name).unwrap(),
            Node {
                flow,
                connections,
                costs: HashMap::new(),
            },
        );
    }

    // Copy to read from to satisfy rusts borrow checker
    let nodes_copy = nodes.clone();

    // Calculate costs in each node
    for (name, node) in nodes.iter_mut() {
        // Free to travel to ourselves
        node.costs.insert(*name, 0);

        // Loop until cost to all nodes are known. BFS
        let mut next_cost = 0;
        while node.costs.len() < nodes_copy.len() {
            let mut tmp = HashMap::new();
            node.costs
                .iter()
                .filter(|(_, c)| **c == next_cost)
                .for_each(|(n, _)| {
                    nodes_copy
                        .get(n)
                        .unwrap()
                        .connections
                        .iter()
                        .filter(|next| !node.costs.contains_key(*next))
                        .for_each(|next| {
                            tmp.insert(*next, next_cost + 1);
                        });
                });
            node.costs.extend(tmp.drain());
            next_cost += 1;
        }
    }

    // Now that we know the costs to travel to any node we can remove redundant ones
    // i.e. all where flow == 0 and not AA as it is our starting point.
    nodes.retain(|name, node| node.flow > 0 || name == name_map.get("AA").unwrap());
    let nodes_copy = nodes.clone();
    for (_, node) in nodes.iter_mut() {
        node.costs.retain(|name, _| nodes_copy.contains_key(name));
    }

    // Task 1
    let aa = *name_map.get("AA").unwrap();
    let visited = BTreeSet::from([aa]);
    let score = recursive_solve(&nodes, aa, 30, visited.clone());
    println!("{}", score);

    // Task 2 beyond

    // k decides the split of work each player should take on, the amount of work
    // is equal to n choose k, for n = 16. Since an even split at 8 is most fair,
    // this is most likely to yield the best result, but it is also the most work.
    let k = 8..9;

    let start_time = Instant::now();
    eprint!("Constructing visiting sets...");

    let mut visited_both = HashSet::new();
    for i in 0..u16::MAX {
        let d: Vec<_> = (0..16).map(|v| (i >> v) % 2).collect();
        let diff: u16 = d.iter().sum();
        if !k.contains(&(diff as usize)) {
            continue;
        }

        let mut visited_1 = visited.clone();
        let mut visited_2 = visited.clone();
        nodes.iter().enumerate().for_each(|(i, (id, _))| {
            if d[i] == 0 {
                visited_1.insert(*id);
            } else {
                visited_2.insert(*id);
            }
        });

        visited_both.insert((visited_1, visited_2));
    }
    let size = visited_both.len();

    // Since this is a large problem and is easily parallelisable we go with a threaded approach.
    // Yes, it's basically a bandaid to a bad solution but I rather it take 10 minutes than an hour.

    // Minus one because the main thread won't be doing any work
    let cores = Into::<usize>::into(thread::available_parallelism().unwrap()) - 1;
    // Who uses single core stuff anyway
    assert!(cores >= 1);

    // Divide the work for the threads. Ideally, the threads would fetch work on demand but instead
    // we split the work evenly at the start.
    let mut split_work: Vec<Vec<_>> = vec![vec![]; cores];
    for (i, work) in visited_both.into_iter().enumerate() {
        split_work[i % cores].push(work);
    }
    let end = start_time.elapsed();
    eprintln!("   ...Done! Took {} ms", end.as_millis());

    let (tx, rx) = mpsc::channel::<(Duration, usize)>();
    for work in split_work {
        let local_nodes = nodes.clone();
        let tx_local = tx.clone();
        thread::spawn(move || {
            for (set1, set2) in work {
                let start = Instant::now();
                let it = recursive_solve_multi(&local_nodes, aa, 26, aa, 26, set1, set2);
                let end = start.elapsed();

                tx_local.send((end, it)).unwrap();
            }
        });
    }
    drop(tx);

    let mut num = 0;
    let mut max_score = 0;
    let mut max_score_num = 0;
    while let Ok((time, it)) = rx.recv() {
        if it > max_score {
            max_score_num = 0;
            max_score = it;
        }
        if max_score == it {
            max_score_num += 1;
        }

        // Print progress info to stderr
        eprintln!("{num:>5}/{size:<5}   Time: {:>5} ms   It: {it:>4}    Max: {max_score:>4} ({max_score_num})", time.as_millis());

        num += 1;
    }
    println!("{}", max_score);
}

fn recursive_solve_multi(
    nodes: &HashMap<usize, Node>,
    cur_id_1: usize,
    mut time_left_1: usize,
    cur_id_2: usize,
    mut time_left_2: usize,
    mut visited_1: BTreeSet<usize>,
    mut visited_2: BTreeSet<usize>,
) -> usize {
    let mut score = 0;

    let cur_node_1 = nodes.get(&cur_id_1).unwrap();
    let cur_node_2 = nodes.get(&cur_id_2).unwrap();
    if cur_node_1.flow > 0 {
        time_left_1 -= 1;
        score += cur_node_1.flow * time_left_1;
        visited_1.insert(cur_id_1);
    }
    if cur_node_2.flow > 0 {
        time_left_2 -= 1;
        score += cur_node_2.flow * time_left_2;
        visited_2.insert(cur_id_2);
    }

    // Break early if we have no chance of getting any more points.
    // This is a suprisingly good optimisation since every branch will end up here
    // (or the single recursive variant) and we will skip iterating through the entire
    // node structure when it's not needed.
    if time_left_1 <= 1 && time_left_2 <= 1 {
        return score;
    }

    // Use multi resolver if both players have valid paths.
    let both_score = nodes
        .iter()
        .filter(|(id1, _)| {
            !visited_1.contains(id1) && cur_node_1.costs.get(*id1).unwrap() < &time_left_1
        })
        .map(|(id1, _)| {
            nodes
                .iter()
                .filter(|(id2, _)| {
                    !visited_2.contains(id2) && cur_node_2.costs.get(*id2).unwrap() < &time_left_2
                })
                .map(|(id2, _)| {
                    recursive_solve_multi(
                        nodes,
                        *id1,
                        time_left_1 - cur_node_1.costs.get(id1).unwrap(),
                        *id2,
                        time_left_2 - cur_node_2.costs.get(id2).unwrap(),
                        visited_1.clone(),
                        visited_2.clone(),
                    )
                })
        })
        .flatten()
        .max()
        .unwrap_or(0);
    score += both_score;

    // Make sure player 1 exhausts all options if player 2 is done.
    score += if both_score == 0 && time_left_1 > 1 && visited_1.len() < nodes.len() {
        nodes
            .iter()
            .filter(|(id1, _)| {
                !visited_1.contains(id1) && cur_node_1.costs.get(*id1).unwrap() < &time_left_1
            })
            .map(|(id1, _)| {
                recursive_solve(
                    nodes,
                    *id1,
                    time_left_1 - cur_node_1.costs.get(id1).unwrap(),
                    visited_1.clone(),
                )
            })
            .max()
            .unwrap_or(0)
    } else {
        0
    };
    // Make sure player 2 exhausts all options if player 1 is done.
    score += if both_score == 0 && time_left_2 > 1 && visited_2.len() < nodes.len() {
        nodes
            .iter()
            .filter(|(id2, _)| {
                !visited_2.contains(id2) && cur_node_2.costs.get(*id2).unwrap() < &time_left_2
            })
            .map(|(id2, _)| {
                recursive_solve(
                    nodes,
                    *id2,
                    time_left_2 - cur_node_2.costs.get(id2).unwrap(),
                    visited_2.clone(),
                )
            })
            .max()
            .unwrap_or(0)
    } else {
        0
    };

    score
}

fn recursive_solve(
    nodes: &HashMap<usize, Node>,
    cur_id: usize,
    mut time_left: usize,
    mut visited: BTreeSet<usize>,
) -> usize {
    let mut score = 0;
    let cur_node = nodes.get(&cur_id).unwrap();
    if cur_node.flow > 0 {
        time_left -= 1;
        score += cur_node.flow * time_left;
        visited.insert(cur_id);
    }

    if time_left <= 1 {
        return score;
    }

    score += nodes
        .iter()
        .filter(|(id, _)| !visited.contains(id) && cur_node.costs.get(*id).unwrap() < &time_left)
        .map(|(id, _)| {
            recursive_solve(
                nodes,
                *id,
                time_left - cur_node.costs.get(id).unwrap(),
                visited.clone(),
            )
        })
        .max()
        .unwrap_or(0);

    score
}
