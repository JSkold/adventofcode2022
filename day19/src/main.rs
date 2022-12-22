use std::{
    cell::RefCell,
    cmp::max,
    collections::BTreeMap,
    io::stdin,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Clone, Copy, Debug)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn zero() -> Resources {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn all_greater_or_equal(&self, rhs: &Self) -> bool {
        self.ore >= rhs.ore
            && self.clay >= rhs.clay
            && self.obsidian >= rhs.obsidian
            && self.geode >= rhs.geode
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
        self
    }
}

impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
        self
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

#[derive(Clone, Debug)]
struct Blueprint {
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl Blueprint {
    fn can_afford_ore_robot(&self, resources: &Resources) -> bool {
        resources.all_greater_or_equal(&self.ore_robot_cost)
    }
    fn can_afford_clay_robot(&self, resources: &Resources) -> bool {
        resources.all_greater_or_equal(&self.clay_robot_cost)
    }
    fn can_afford_obsidian_robot(&self, resources: &Resources) -> bool {
        resources.all_greater_or_equal(&self.obsidian_robot_cost)
    }
    fn can_afford_geode_robot(&self, resources: &Resources) -> bool {
        resources.all_greater_or_equal(&self.geode_robot_cost)
    }
}

#[derive(Clone)]
struct SimulationState<'a> {
    pruner: &'a Pruner,
    blueprint: &'a Blueprint,
    // Inventory of resources
    resources: Resources,
    // Delta resouces per turn
    delta: Resources,
    time_left: usize,
}

struct Pruner {
    simulation_time: usize,
    best_score: RefCell<usize>,
}

impl Pruner {
    fn prune_simulation(&self, state: &SimulationState) -> Result<(), ()> {
        self.register_best_score(state);

        // Static prunes, doesn't compare state between different simulations.
        Pruner::no_obsidian_7_turns_left(state)?;
        Pruner::more_than_100_clay(state)?;
        Pruner::more_than_50_ore(state)?;

        self.build_ore_or_clay_robot_asap(state)?;
        self.cant_beat_best_score(state)?;

        Ok(())
    }

    fn register_best_score(&self, state: &SimulationState) {
        if state.resources.geode > *self.best_score.borrow() {
            *self.best_score.borrow_mut() = state.resources.geode;
        }
    }

    /// If no obsidian gathering has begun when there is less 7 turns left
    /// the simulation is unlikely to be optimal.
    fn no_obsidian_7_turns_left(state: &SimulationState) -> Result<(), ()> {
        if state.time_left < 7 && state.delta.obsidian == 0 {
            Err(())?
        }
        Ok(())
    }

    /// If the simulation exceeds 100 clay.
    fn more_than_100_clay(state: &SimulationState) -> Result<(), ()> {
        if state.resources.clay >= 100 {
            Err(())?
        }
        Ok(())
    }

    // If the simulation exceeds 50 ore.
    fn more_than_50_ore(state: &SimulationState) -> Result<(), ()> {
        if state.resources.ore >= 50 {
            Err(())?
        }
        Ok(())
    }

    // If the simulation doesn't invest immediately.
    fn build_ore_or_clay_robot_asap(&self, state: &SimulationState) -> Result<(), ()> {
        if state.time_left
            == self.simulation_time
                - 2
                - max(
                    // Ore and clay robots only cost ore
                    state.blueprint.ore_robot_cost.ore,
                    state.blueprint.clay_robot_cost.ore,
                )
        {
            if state.delta.ore == 1 && state.delta.clay == 0 {
                // No ore nor clay robots have been built on the first possible turn.
                Err(())?;
            }
        }
        Ok(())
    }

    // If the simulation can't beat the best score of all other simulations.
    fn cant_beat_best_score(&self, state: &SimulationState) -> Result<(), ()> {
        if state.time_left <= 7 {
            if (0..=state.time_left)
                .into_iter()
                .map(|n| n + state.delta.geode)
                .sum::<usize>()
                + state.resources.geode
                <= *self.best_score.borrow()
            {
                Err(())?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut blueprints: BTreeMap<usize, Blueprint> = BTreeMap::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut split = line.split(' ');
        let id = split
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse::<usize>()
            .unwrap();
        let ore_robot_ore_cost = split.nth(4).unwrap().parse::<usize>().unwrap();
        let clay_robot_ore_cost = split.nth(5).unwrap().parse::<usize>().unwrap();
        let obsidian_robot_ore_cost = split.nth(5).unwrap().parse::<usize>().unwrap();
        let obsidian_robot_clay_cost = split.nth(2).unwrap().parse::<usize>().unwrap();
        let geode_robot_ore_cost = split.nth(5).unwrap().parse::<usize>().unwrap();
        let geode_robot_obsidian_cost = split.nth(2).unwrap().parse::<usize>().unwrap();

        let bp = Blueprint {
            ore_robot_cost: Resources {
                ore: ore_robot_ore_cost,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_robot_cost: Resources {
                ore: clay_robot_ore_cost,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_robot_cost: Resources {
                ore: obsidian_robot_ore_cost,
                clay: obsidian_robot_clay_cost,
                obsidian: 0,
                geode: 0,
            },
            geode_robot_cost: Resources {
                ore: geode_robot_ore_cost,
                clay: 0,
                obsidian: geode_robot_obsidian_cost,
                geode: 0,
            },
        };
        blueprints.insert(id, bp);
    }

    let total = blueprints
        .clone()
        .into_iter()
        .map(|(id, blueprint)| {
            let pruner = Pruner {
                simulation_time: 24,
                best_score: RefCell::new(0),
            };
            let initial_state = SimulationState {
                pruner: &pruner,
                blueprint: &blueprint,
                resources: Resources {
                    ore: 0,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                delta: Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                time_left: 24,
            };

            id * simulate_blueprint(initial_state).geode
        })
        .sum::<usize>();

    println!("{}", total);

    let threads: Vec<_> = blueprints
        .into_iter()
        .take(3)
        .map(|(_, blueprint)| {
            std::thread::spawn(move || -> usize {
                let pruner = Pruner {
                    simulation_time: 32,
                    best_score: RefCell::new(0),
                };
                let initial_state = SimulationState {
                    pruner: &pruner,
                    blueprint: &blueprint,
                    resources: Resources {
                        ore: 0,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    delta: Resources {
                        ore: 1,
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    time_left: 32,
                };

                simulate_blueprint(initial_state).geode
            })
        })
        .collect();

    let total2 = threads
        .into_iter()
        .map(|j| j.join().unwrap())
        .product::<usize>();

    println!("{}", total2);

    // 1566 too low
}

fn simulate_blueprint(mut state: SimulationState) -> Resources {
    // Step time
    state.time_left -= 1;
    if state.time_left == 0 {
        // No decisions going to effect outcome, return delta.
        return state.delta;
    }

    // Since this problem is huge and many branches are worthless we can speed
    // up the simulation substantially. The pruner looks at the simulation state
    // and applies some heuristics to determine if execution on this branch should continue.
    if state.pruner.prune_simulation(&state).is_err() {
        return Resources::zero();
    }

    // We have 5 decisions we can take:
    // 1. Build ore robot
    // 2. Build clay robot
    // 3. Build obsidian robot
    // 4. Build geode robot
    // 5. Wait
    // Decisions 1-4 are not always avaliable.
    // Decision 5 is always possible BUT we will always want to
    // reinvest any resources we have into more robots.
    let mut decisions: Vec<SimulationState> = Vec::new();
    if state.blueprint.can_afford_ore_robot(&state.resources) {
        let mut next_state = state.clone();
        next_state.delta.ore += 1;
        next_state.resources -= state.blueprint.ore_robot_cost;
        decisions.push(next_state);
    }
    if state.blueprint.can_afford_clay_robot(&state.resources) {
        let mut next_state = state.clone();
        next_state.delta.clay += 1;
        next_state.resources -= state.blueprint.clay_robot_cost;
        decisions.push(next_state);
    }
    if state.blueprint.can_afford_obsidian_robot(&state.resources) {
        let mut next_state = state.clone();
        next_state.delta.obsidian += 1;
        next_state.resources -= state.blueprint.obsidian_robot_cost;
        decisions.push(next_state);
    }
    if state.blueprint.can_afford_geode_robot(&state.resources) {
        let mut next_state = state.clone();
        next_state.delta.geode += 1;
        next_state.resources -= state.blueprint.geode_robot_cost;
        decisions.push(next_state);
    }
    // If decisions < 4 we can't afford all robots and waiting is a valid branch.
    if decisions.len() < 4 {
        decisions.push(state.clone());
    }

    let max_geode = decisions
        .into_iter()
        .map(|mut next| {
            // Apply income for next state
            next.resources += state.delta;
            simulate_blueprint(next)
        })
        .max_by(|l, r| l.geode.cmp(&r.geode))
        // Safe unwrap since waiting is always an option if the other decisions aren't avaliable.
        .unwrap();

    state.delta + max_geode
}
