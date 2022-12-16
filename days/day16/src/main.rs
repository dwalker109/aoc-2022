use std::collections::{HashMap, HashSet, BTreeSet};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let tunnels = Tunnels::from(input);

    let mut results = Vec::new();
    let mut cache = BTreeSet::new();

    tunnels.walk_multiverse("AA", "AA", BTreeSet::new(), 30, 0, &mut results, &mut cache);

    *results.iter().max().unwrap()
}

fn part2(input: &'static str) -> usize {
    todo!();
}

#[derive(Debug)]
struct Tunnels(HashMap<String, Tunnel>);

impl From<&str> for Tunnels {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .map(|l| {
                    let tunnel = Tunnel::from(l);

                    (tunnel.id.clone(), tunnel)
                })
                .collect(),
        )
    }
}

impl Tunnels {
    fn walk_multiverse(
        &self,
        current_location: &str,
        prev_location: &str,
        open: BTreeSet<String>,
        mut time_remaining: u8,
        released: usize,
        results: &mut Vec<usize>,
        cache: &mut BTreeSet<(u8, usize, String)>,
    ) {
        if time_remaining == 0 {
            results.push(released);
            return;
        }

        let t = self.0.get(current_location).unwrap();
        let key = (time_remaining, released, t.id.clone());

        if cache.contains(&key) {
            return;
        } else {
            cache.insert(key);
        }

        // Tick
        time_remaining -= 1;

        // Open self if possible and sensible
        if t.flow_rate > 0 && !open.contains(&t.id) {
            let mut open = open.clone();
            open.insert(t.id.clone());
            self.walk_multiverse(
                current_location,
                current_location,
                open,
                time_remaining,
                released + (t.flow_rate * time_remaining as usize),
                results,
                cache,
            );
        }

        // Visit connections
        for id in t.connections.iter().filter(|&c| c != prev_location) {
            self.walk_multiverse(
                id,
                current_location,
                open.clone(),
                time_remaining,
                released,
                results,
                cache,
            );
        }
    }
}

#[derive(Debug)]
struct Tunnel {
    id: String,
    flow_rate: usize,
    connections: Vec<String>,
}

impl From<&str> for Tunnel {
    fn from(value: &str) -> Self {
        let id = value.split_ascii_whitespace().nth(1).unwrap().into();

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

        let connections = value
            .split_ascii_whitespace()
            .skip(9)
            .map(|x| x.trim_end_matches(',').into())
            .collect();

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
        assert_eq!(super::part2(INPUT), 0);
    }
}
