use std::{
    collections::{BinaryHeap, HashMap},
    sync::RwLock,
};

use monkey::{Id, Monkey, MonkeyReduce};

static INPUT: &str = include_str!("../input");

mod monkey;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    solve::<20>(input, MonkeyReduce::DivThree)
}

fn part2(input: &'static str) -> usize {
    solve::<10000>(input, MonkeyReduce::Lcm(get_supermodulo(input)))
}

fn solve<const N: usize>(input: &'static str, mr: MonkeyReduce) -> usize {
    let monkeys = get_monkeys(input);
    let order = monkeys.keys().collect::<BinaryHeap<_>>().into_sorted_vec();

    for _ in 1..=N {
        for id in order.iter() {
            let current = monkeys.get(id).unwrap();
            while let Some((recipient, val)) = current.write().unwrap().throw(&mr) {
                monkeys.get(&recipient).unwrap().write().unwrap().catch(val);
            }
        }
    }

    let mut inspected = monkeys
        .values()
        .map(|m| m.read().unwrap().inspected)
        .collect::<Vec<_>>();

    inspected.sort_unstable();
    inspected.reverse();

    inspected.iter().take(2).product()
}

fn get_monkeys(input: &'static str) -> HashMap<Id, RwLock<Monkey>> {
    input
        .split("\n\n")
        .map(|m_raw| {
            let monkey = RwLock::new(Monkey::from(m_raw));
            let id = monkey.read().unwrap().id;
            (id, monkey)
        })
        .collect()
}

fn get_supermodulo(input: &'static str) -> usize {
    input
        .lines()
        .filter(|l| l.contains("divisible by"))
        .filter_map(|l| {
            l.split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .ok()
        })
        .product()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 10605)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2713310158)
    }
}
