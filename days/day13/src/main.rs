use std::{cmp::Ordering, str::FromStr};

use serde_json::Value;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let pairs = parse(input);

    pairs
        .enumerate()
        .filter_map(|(i, [l, r])| matches!(cmp(&l, &r), Ordering::Less).then(|| i + 1))
        .sum()
}

fn part2(input: &'static str) -> usize {
    let dividers = [Value::from_str("[[2]]").unwrap(), Value::from_str("[[6]]").unwrap()];

    let mut pairs = parse(input)
        .flatten()
        .chain(dividers.clone().into_iter())
        .collect::<Vec<_>>();

    pairs.sort_by(cmp);

    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, v)| dividers.contains(&v).then(|| i + 1))
        .product()
}

fn parse(input: &'static str) -> impl Iterator<Item = [Value; 2]> {
    input.split("\n\n").map(|p| {
        <[Value; 2]>::try_from(
            p.lines()
                .map(|l| serde_json::from_str::<Value>(l).unwrap())
                .collect::<Vec<_>>(),
        )
        .unwrap()
    })
}

fn cmp(l: &Value, r: &Value) -> Ordering {
    if l.is_array() && r.is_array() {
        let lv = l.as_array().unwrap();
        let rv = r.as_array().unwrap();

        for i in 0..std::cmp::max(lv.len(), rv.len()) {
            let (lv, rv) = (lv.get(i), rv.get(i));

            if rv.is_none() {
                return Ordering::Greater;
            }

            if lv.is_none() {
                return Ordering::Less;
            }

            let next = cmp(lv.unwrap(), rv.unwrap());

            if matches!(next, Ordering::Equal) {
                continue;
            }

            return next;
        }

        return Ordering::Equal;
    }

    if l.is_number() && r.is_number() {
        return l.as_i64().unwrap().cmp(&r.as_i64().unwrap());
    }

    if l.is_number() && r.is_array() {
        return cmp(&Value::from(vec![l.as_i64().unwrap()]), r);
    }

    if l.is_array() && r.is_number() {
        return cmp(l, &Value::from(vec![r.as_i64().unwrap()]));
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 140);
    }
}
