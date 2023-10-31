#![feature(iter_next_chunk)]
#![feature(int_roundings)]

use blueprint::Blueprint;
use state::State;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let blueprints = input.lines().map(Blueprint::from).collect::<Vec<_>>();

    calc_finished_states(blueprints, 24)
        .map(|(b, s)| {
            s.iter()
                .max_by_key(|v| v.materials.geode)
                .unwrap()
                .materials
                .geode
                * b.id
        })
        .sum()
}

fn part2(input: &'static str) -> usize {
    let blueprints = input
        .lines()
        .take(3)
        .map(Blueprint::from)
        .collect::<Vec<_>>();

    calc_finished_states(blueprints, 32)
        .map(|(_, s)| {
            s.iter()
                .max_by_key(|v| v.materials.geode)
                .unwrap()
                .materials
                .geode
        })
        .product()
}

fn calc_finished_states(
    blueprints: Vec<Blueprint>,
    mins: usize,
) -> impl Iterator<Item = (Blueprint, Vec<State>)> {
    blueprints.into_iter().map(move |blueprint| {
        let mut states = Vec::new();
        let mut finished = Vec::new();
        states.push(State::new(mins));

        loop {
            let mut next_states = Vec::new();

            for s in &states {
                let (settled, in_progress): (Vec<_>, Vec<_>) =
                    s.next(&blueprint).partition(|s| s.remaining == 0);

                finished.extend(settled);
                next_states.extend(in_progress);
            }

            if next_states.is_empty() {
                break;
            }

            states = next_states;
        }

        (blueprint, finished)
    })
}

mod blueprint;
mod state;

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 33);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 56 * 62);
    }
}
