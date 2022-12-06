#![feature(array_windows)]
#![feature(control_flow_enum)]

use std::{collections::HashSet, ops::ControlFlow};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    locate_unique_pos::<4>(input)
}

fn part2(input: &'static str) -> usize {
    locate_unique_pos::<14>(input)
}

fn locate_unique_pos<const N: usize>(input: &'static str) -> usize {
    let mut uniques = HashSet::with_capacity(N);

    let found = input
        .chars()
        .collect::<Vec<_>>()
        .array_windows::<N>()
        .try_fold(N, |acc, curr| {
            if curr.iter().all(|c| uniques.insert(c)) {
                ControlFlow::Break(acc)
            } else {
                uniques.clear();
                ControlFlow::Continue(acc + 1)
            }
        });

    found.break_value().unwrap()
}

#[cfg(test)]
mod tests {
    static INPUT: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT[0]), 7);
        assert_eq!(super::part1(INPUT[1]), 5);
        assert_eq!(super::part1(INPUT[2]), 6);
        assert_eq!(super::part1(INPUT[3]), 10);
        assert_eq!(super::part1(INPUT[4]), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT[0]), 19);
        assert_eq!(super::part2(INPUT[1]), 23);
        assert_eq!(super::part2(INPUT[2]), 23);
        assert_eq!(super::part2(INPUT[3]), 29);
        assert_eq!(super::part2(INPUT[4]), 26);
    }
}
