#![feature(control_flow_enum)]

use std::{collections::HashMap, ops::ControlFlow};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let trees = Trees::from(input);

    trees
        .grid
        .iter()
        .filter(|(&xy, _)| trees.is_ext_visible(xy))
        .count()
}

fn part2(input: &'static str) -> usize {
    let trees = Trees::from(input);

    trees
        .grid
        .iter()
        .map(|(&xy, _)| trees.calc_scenic_score(xy))
        .max()
        .unwrap() as usize
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Xy(usize, usize);

struct Trees {
    grid: HashMap<Xy, u8>,
    max_dim: usize,
}

impl From<&str> for Trees {
    fn from(raw: &str) -> Self {
        let max_dim = raw.lines().count() - 1;

        let mut grid = HashMap::with_capacity(max_dim * max_dim);

        for (y, l) in raw.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                grid.insert(Xy(x, y), u8::try_from(c.to_digit(10).unwrap()).unwrap());
            }
        }

        Self {
            grid,
            max_dim
        }
    }
}

impl Trees {
    fn is_ext_visible(&self, Xy(sx, sy): Xy) -> bool {
        let l_path = (0..=sx).map(|cx| self.grid.get(&Xy(cx, sy)).unwrap()).rev();
        let r_path = (sx..=self.max_dim).map(|cx| self.grid.get(&Xy(cx, sy)).unwrap());
        let t_path = (0..=sy).map(|cy| self.grid.get(&Xy(sx, cy)).unwrap()).rev();
        let b_path = (sy..=self.max_dim).map(|cy| self.grid.get(&Xy(sx, cy)).unwrap());

        fn do_calc<'a>(mut l: impl Iterator<Item = &'a u8>) -> bool {
            let x = l.next().unwrap();

            l.all(|y| y < x)
        }

        do_calc(l_path)
            || do_calc(r_path)
            || do_calc(t_path)
            || do_calc(b_path)
    }

    fn calc_scenic_score(&self, Xy(sx, sy): Xy) -> usize {
        let l_path = (0..=sx).map(|cx| self.grid.get(&Xy(cx, sy)).unwrap()).rev();
        let r_path = (sx..=self.max_dim).map(|cx| self.grid.get(&Xy(cx, sy)).unwrap());
        let t_path = (0..=sy).map(|cy| self.grid.get(&Xy(sx, cy)).unwrap()).rev();
        let b_path = (sy..=self.max_dim).map(|cy| self.grid.get(&Xy(sx, cy)).unwrap());

        fn do_calc<'a>(mut l: impl Iterator<Item = &'a u8>) -> usize {
            let x = l.next().unwrap();

            let res = l.try_fold(0, |mut acc, cur| {
                acc += 1;
                if cur >= x {
                    ControlFlow::Break(acc)
                } else {
                    ControlFlow::Continue(acc)
                }
            });

            res.break_value().or(res.continue_value()).unwrap()
        }

        do_calc(l_path) * do_calc(r_path) * do_calc(t_path) * do_calc(b_path)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 8);
    }
}
