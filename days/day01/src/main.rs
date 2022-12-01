use std::collections::BinaryHeap;

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    let r1 = part1(INPUT);
    let r2 = part2(INPUT);

    println!("Part 1: {r1}");
    println!("Part 2: {r2}");
}

#[logging_timer::time]
fn part1(input: &'static str) -> usize {
    cals_per_elve(input)
        .fold(0, |acc, el| std::cmp::max(acc, el))
}

#[logging_timer::time]
fn part2(input: &'static str) -> usize {
    cals_per_elve(input).collect::<BinaryHeap<_>>().into_sorted_vec().into_iter().rev().take(3).sum()
}

fn cals_per_elve(input: &'static str) -> impl Iterator<Item = usize> {
    input
    .split("\n\n")
    .map(|el| {
        el.lines()
            .fold(0, |acc, el| acc + el.parse::<usize>().unwrap())
    })

}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 24000);
    }
    
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 45000);
    }
}
