#![feature(binary_heap_into_iter_sorted)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> String {
    let (mut crates, instr) = parse(input);

    for (qty, from, to) in instr {
        for _ in 0..qty {
            let moving = crates.get_mut(&from).unwrap().pop_back().unwrap();
            crates.get_mut(&to).unwrap().push_back(moving);
        }
    }

    checksum(crates)
}

fn part2(input: &'static str) -> String {
    let (mut crates, instr) = parse(input);

    for (qty, from, to) in instr {
        let source = crates.get_mut(&from).unwrap();
        let mut moving = source.split_off(source.len() - qty);
        crates.get_mut(&to).unwrap().append(&mut moving);
    }

    checksum(crates)
}

fn parse(
    input: &'static str,
) -> (
    HashMap<usize, VecDeque<char>>,
    impl Iterator<Item = (usize, usize, usize)>,
) {
    let (crates_in, instr_in) = input.split_once("\n\n").unwrap();

    let mut crates_init = HashMap::<_, VecDeque<_>>::new();
    for l in crates_in.lines() {
        for (n, c) in l.chars().enumerate().filter(|(_, c)| c.is_alphanumeric()) {
            crates_init
                .entry(n)
                .and_modify(|s| s.push_front(c))
                .or_insert_with(|| vec![c].into());
        }
    }

    let mut crates = HashMap::new();
    for mut s in crates_init.into_values() {
        let pos = s.pop_front().unwrap();
        crates.insert(pos.to_digit(10).unwrap() as usize, s);
    }

    let instr = instr_in.lines().map(|l| {
        let mut i = l.split_whitespace().filter_map(|w| w.parse::<usize>().ok());
        (i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
    });

    (crates, instr)
}

fn checksum(crates: HashMap<usize, VecDeque<char>>) -> String {
    crates
        .keys()
        .map(|n| Reverse(*n))
        .collect::<BinaryHeap<_>>()
        .into_iter_sorted()
        .fold(String::with_capacity(crates.len()), |mut acc, curr| {
            acc.push(*crates.get(&curr.0).unwrap().iter().last().unwrap());
            acc
        })
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), "CMZ".to_string());
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), "MCD".to_string());
    }
}
