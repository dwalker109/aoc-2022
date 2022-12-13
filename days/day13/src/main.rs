use std::cmp::Ordering;

use serde_json::Value;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let pairs = input.split("\n\n").map(|p| {
        <[Value; 2]>::try_from(
            p.lines()
                .map(|l| serde_json::from_str::<Value>(l).unwrap())
                .collect::<Vec<_>>(),
        )
        .unwrap()
    });

    pairs
        .enumerate()
        .filter_map(|(i, [l, r])| matches!(compare(l, r), Sorted::Yes).then(|| i + 1))
        .sum()
}

#[derive(Debug)]
enum Sorted {
    Yes,
    No,
    Undecided,
}

fn compare(l: Value, r: Value) -> Sorted {
    println!("Comparing {} with {}", l, r);

    if l.is_array() && r.is_array() {
        let lv = l.as_array().unwrap();
        let rv = r.as_array().unwrap();

        for i in 0..std::cmp::max(lv.len(), rv.len()) {
            let (lv, rv) = (lv.get(i), rv.get(i));

            if rv.is_none() {
                return Sorted::No;
            }

            if lv.is_none() {
                return Sorted::Yes;
            }

            let next = compare(lv.unwrap().to_owned(), rv.unwrap().to_owned());

            if matches!(next, Sorted::Undecided) {
                continue;
            }

            return next;
        }

        return Sorted::Undecided;
    }

    if l.is_number() && r.is_number() {
        return match l.as_i64().unwrap().cmp(&r.as_i64().unwrap()) {
            Ordering::Less => Sorted::Yes,
            Ordering::Equal => Sorted::Undecided,
            Ordering::Greater => Sorted::No,
        };
    }

    if l.is_number() && r.is_array() {
        return compare(Value::from(vec![l.as_i64().unwrap()]), r);
    }

    if l.is_array() && r.is_number() {
        return compare(l, Value::from(vec![r.as_i64().unwrap()]));
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13)
    }

    #[test]
    fn part1_ex() {
        assert_eq!(super::part1("[[],8]\n[[3]]"), 1);
        // assert_eq!(super::part1(INPUT), 13)
        // assert_eq!(super::part1(INPUT), 13)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 0)
    }
}
