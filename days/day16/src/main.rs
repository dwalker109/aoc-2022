use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let mut tunnels = Tunnels::from(input);

    tunnels.walk_multiverse(0x00, 0x00, ValveState(0), 30, 0);
    *tunnels.results.values().max().unwrap()
}

fn part2(input: &'static str) -> usize {
    let mut tunnels = Tunnels::from(input);

    tunnels.walk_multiverse(0x00, 0x00, ValveState(0), 26, 0);

    let mut best = 0;
    for (l_state, l_val) in tunnels.results.iter() {
        for (r_state, r_val) in tunnels.results.iter() {
            if l_state.is_disjoint(r_state) {
                best = std::cmp::max(best, l_val + r_val);
            }
        }
    }

    best
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct ValveState(u64);

impl ValveState {
    fn is_open(&self, id: &u8) -> bool {
        self.0 & (1 << id) != 0
    }

    fn open(&mut self, id: u8) {
        self.0 |= 1 << id;
    }

    fn is_disjoint(&self, other: &ValveState) -> bool {
        self.0 & other.0 == 0
    }
}

#[derive(Debug)]
struct Tunnels {
    valves: HashMap<u8, Valve>,
    cache: HashSet<(u8, usize, u8, ValveState)>,
    results: HashMap<ValveState, usize>,
}

impl From<&str> for Tunnels {
    fn from(value: &str) -> Self {
        let mut value_string = String::from(value);
        let mut next_id = 0u8;
        for l in value.lines() {
            let id_string = &l[6..=7];

            if id_string == "AA" {
                // Ensure AA get id 0 so we can start from it later
                value_string = value_string.replace(id_string, &0x00.to_string());
            } else {
                next_id += 1;
                value_string = value_string.replace(id_string, &next_id.to_string());
            }
        }

        let valves = value_string
            .lines()
            .map(|l| {
                let tunnel = Valve::from(l);

                (tunnel.id, tunnel)
            })
            .collect::<HashMap<_, _>>();

        Self {
            valves,
            cache: HashSet::new(),
            results: HashMap::new(),
        }
    }
}

impl Tunnels {
    fn walk_multiverse(
        &mut self,
        current_location: u8,
        prev_location: u8,
        state: ValveState,
        mut time_remaining: u8,
        released: usize,
    ) {
        if time_remaining == 0 {
            let e = self.results.entry(state).or_default();
            if *e < released {
                *e = released;
            }

            return;
        }

        let t = *self.valves.get(&current_location).unwrap();
        let key = (time_remaining, released, t.id, state);

        if self.cache.contains(&key) {
            return;
        } else {
            self.cache.insert(key);
        }

        // Tick
        time_remaining -= 1;

        // Open self if possible and sensible
        if t.flow_rate > 0 && !state.is_open(&t.id) {
            let mut state = state;
            state.open(t.id);
            self.walk_multiverse(
                current_location,
                current_location,
                state,
                time_remaining,
                released + (t.flow_rate * time_remaining as usize),
            );
        }

        // Visit connections
        for id in t
            .connections
            .iter()
            .flatten()
            .filter(|&c| c != &prev_location)
        {
            self.walk_multiverse(*id, current_location, state, time_remaining, released);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Valve {
    id: u8,
    flow_rate: usize,
    connections: [Option<u8>; 8],
}

impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        let id = value
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let flow_rate = value
            .split_ascii_whitespace()
            .nth(4)
            .map(|x| {
                x.chars()
                    .filter(|x| x.is_numeric())
                    .collect::<String>()
                    .parse()
                    .unwrap()
            })
            .unwrap();

        let connections_vec = value
            .split_ascii_whitespace()
            .skip(9)
            .map(|x| x.trim_end_matches(',').parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        let mut connections = [None; 8];
        for (idx, id) in connections_vec.into_iter().enumerate() {
            connections[idx] = Some(id);
        }

        Self {
            id,
            flow_rate,
            connections,
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1651);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1707);
    }

    #[test]
    fn bits() {
        let a: u64 = 0b0000000000000000000000000010010000000000000001000001000000000100;
        let b: u64 = 0b0000000000000000001000010100000001000000010000000000000110000010;

        assert!(a & b == 0);
    }
}
