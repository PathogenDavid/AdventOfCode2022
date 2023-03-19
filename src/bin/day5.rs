// Saw this and thought it was a good excuse to mess with condtitional compilation
#![cfg_attr(feature = "nightly", feature(get_many_mut))]

// Regex is overkill here, just wanted an excuse to try the crate
use regex::Regex;

fn main() {
    // Parse the ship manifest
    let manifest = include_str!("day5.txt").trim_end().replace("\r", "");
    let mut manifest = manifest.lines();

    let cargo_hold = CargoHold::new(&mut manifest);

    let blank_line = manifest.next().unwrap();
    assert_eq!(blank_line, "", "data should have blank line between cargo hold map and crane action listing");

    let mut crane_actions = vec![];
    let crane_action_regex = Regex::new(r"^move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();
    for crane_action in manifest {
        let captures = crane_action_regex.captures(crane_action).expect("could not parse crane movement");
        crane_actions.push(CraneAction {
            count: captures["count"].parse().unwrap(),
            // -1 because instructions are one-indexed
            from: captures["from"].parse::<usize>().unwrap() - 1,
            to: captures["to"].parse::<usize>().unwrap() - 1,
        });
    }

    println!("Part 1: {}", run_simulation(CraneModel::CrateMover9000, cargo_hold.clone(), &crane_actions));
    println!("Part 2: {}", run_simulation(CraneModel::CrateMover9001, cargo_hold.clone(), &crane_actions));
}

fn run_simulation(crane: CraneModel, mut cargo_hold: CargoHold, crane_actions: &Vec<CraneAction>) -> String {
    for crane_action in crane_actions {
        match crane {
            CraneModel::CrateMover9000 => cargo_hold.move_crates(crane_action.from, crane_action.to, crane_action.count),
            CraneModel::CrateMover9001 => cargo_hold.move_crates_9001(crane_action.from, crane_action.to, crane_action.count),
        }
    }

    let mut result = String::new();
    for i in 0..cargo_hold.stack_count() {
        if let Some(c) = cargo_hold.peek_stack(i) {
            result.push(*c);
        }
    }

    return result;
}

enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Clone)]
struct CargoHold {
    stacks: Vec<Vec<char>>,
}

impl CargoHold {
    fn new<'a, I>(data: &mut I) -> CargoHold
    where
        I : Iterator<Item = &'a str>,
    {
        let mut stacks = Vec::new();

        // Read in the stack data
        'outer: for line in data.map(|l| l.chars().collect::<Vec<_>>()) {
            for (stack_num, chunk) in line.chunks(4).enumerate() {
                if stack_num >= stacks.len() {
                    stacks.push(Vec::new());
                }

                if chunk[0] == ' ' {
                    if chunk[1] == ' ' {
                        // Empty crate, move to the next one
                        continue;
                    } else {
                        // Number marker, we're done reading the cargo section of the manifest
                        assert!(char::is_ascii_digit(&chunk[1]));
                        break 'outer;
                    }
                }

                // This is a crate, put it in the appropriate stack
                assert_eq!(chunk[0], '[');
                assert_eq!(chunk[2], ']');
                stacks[stack_num].push(chunk[1]);
            }
        }

        // Flip all the stacks over since they're currently upside-down
        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        CargoHold { stacks }
    }

    #[cfg(feature = "nightly")]
    fn move_crates(&mut self, from: usize, to: usize, count: usize) {
        // If the stacks are the same nothing will happen
        if from == to {
            return;
        }

        let [from, to] = self.stacks.get_many_mut([from, to]).unwrap();
        for _ in 0..count {
            if let Some(c) = from.pop() {
                to.push(c);
            }
        }
    }

    #[cfg(not(feature = "nightly"))]
    fn move_crates(&mut self, from: usize, to: usize, count: usize) {
        // If the stacks are the same nothing will happen
        if from == to {
            return;
        }

        for _ in 0..count {
            if let Some(c) = self.stacks[from].pop() {
                self.stacks[to].push(c);
            }
        }
    }

    fn move_crates_9001(&mut self, from: usize, to: usize, count: usize) {
        // If the stacks are the same nothing will happen
        if from == to {
            return;
        }

        let from = &mut self.stacks[from];
        let split = from.len() - count;
        let mut to_move: Vec<_> = from.drain(split..).collect();
        self.stacks[to].append(&mut to_move);
    }

    fn peek_stack(&self, stack_num: usize) -> Option<&char> {
        self.stacks[stack_num].last()
    }

    fn stack_count(&self) -> usize {
        self.stacks.len()
    }
}

#[derive(Clone)]
struct CraneAction {
    count: usize,
    from: usize,
    to: usize,
}
