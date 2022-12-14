#![feature(array_windows)]

use std::{
    cmp::{max, min},
    collections::HashMap,
};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let mut c = Cave::from(input);

    loop {
        if c.try_add_sand::<500>().is_err() {
            return c.count_sand();
        }
    }
}

fn part2(input: &'static str) -> usize {
    let mut c = Cave::from(input);

    for x in (c.min_x - c.max_x / 2)..(c.max_x + c.min_x / 2) {
        c.plan.insert(Xy(x, 2 + c.max_y), Matter::Rock);
    }

    c.max_y = usize::MAX;

    loop {
        if c.try_add_sand::<500>().is_err() {
            return c.count_sand();
        }
    }
}

#[derive(Debug)]
enum Matter {
    Rock,
    Sand,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Xy(usize, usize);

#[derive(Debug)]
struct Cave {
    plan: HashMap<Xy, Matter>,
    max_y: usize,
    min_x: usize,
    max_x: usize,
}

impl Cave {
    fn try_add_sand<const O: usize>(&mut self) -> Result<(), ()> {
        let mut g = Xy(O, 0);

        loop {
            if self.plan.get(&g).is_some() {
                return Err(());
            }

            if g.1 > self.max_y {
                return Err(());
            }

            if self.plan.get(&Xy(g.0, g.1 + 1)).is_none() {
                g = Xy(g.0, g.1 + 1);
                continue;
            }

            if self.plan.get(&Xy(g.0 - 1, g.1 + 1)).is_none() {
                g = Xy(g.0 - 1, g.1 + 1);
                continue;
            }

            if self.plan.get(&Xy(g.0 + 1, g.1 + 1)).is_none() {
                g = Xy(g.0 + 1, g.1 + 1);
                continue;
            }

            break;
        }

        self.plan.insert(g, Matter::Sand);

        Ok(())
    }

    fn count_sand(&self) -> usize {
        self.plan
            .values()
            .filter(|m| matches!(m, Matter::Sand))
            .count()
    }
}

impl From<&str> for Cave {
    fn from(raw: &str) -> Self {
        let mut plan = HashMap::new();

        for line in raw.lines() {
            let coords = line
                .split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();

                    Xy(x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<_>>();

            for [Xy(xa, ya), Xy(xb, yb)] in coords.array_windows::<2>() {
                if xa == xb {
                    for y in min(*ya, *yb)..=max(*ya, *yb) {
                        plan.insert(Xy(*xa, y), Matter::Rock);
                    }
                }

                if ya == yb {
                    for x in min(*xa, *xb)..=max(*xa, *xb) {
                        plan.insert(Xy(x, *ya), Matter::Rock);
                    }
                }
            }
        }

        let max_y = plan.keys().max_by_key(|xy| xy.1).unwrap().1;
        let min_x = plan.keys().min_by_key(|xy| xy.0).unwrap().0;
        let max_x = plan.keys().max_by_key(|xy| xy.0).unwrap().0;

        Self {
            plan,
            max_y,
            min_x,
            max_x,
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 24);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 93);
    }
}
