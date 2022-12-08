#![feature(is_sorted)]
#![feature(array_windows)]

use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let trees = Trees::from(input);

    let mut visible = trees
        .grid
        .iter()
        .filter(|(&xy, _)| trees.is_ext_visible(xy))
        .collect::<Vec<_>>();

    // visible.sort();
    // dbg!(&visible);

    visible.len()
}

fn part2(input: &'static str) -> usize {
    0
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Xy(usize, usize);

struct Trees {
    grid: HashMap<Xy, u8>,
    width: usize,
    height: usize,
}

impl From<&str> for Trees {
    fn from(raw: &str) -> Self {
        let width = raw.lines().count() - 1;
        let height = width;

        let mut grid = HashMap::with_capacity(width * height);

        for (y, l) in raw.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                grid.insert(Xy(x, y), u8::try_from(c.to_digit(10).unwrap()).unwrap());
            }
        }

        Self {
            grid,
            width,
            height,
        }
    }
}

impl Trees {
    fn is_ext_visible(&self, Xy(sx, sy): Xy) -> bool {
        let l_path = (0..=sx).map(|cx| self.grid.get(&Xy(cx, sy)).unwrap()).rev();
        let r_path = (sx..=self.width).map(|cx| self.grid.get(&Xy(cx, sy)).unwrap());
        let t_path = (0..=sy).map(|cy| self.grid.get(&Xy(sx, cy)).unwrap()).rev();
        let b_path = (sy..=self.height).map(|cy| self.grid.get(&Xy(sx, cy)).unwrap());

        let los = |l: Vec<&u8>| l[1..].iter().all(|&x| x < l[0]);

        let los_r = los(l_path.collect())
            || los(r_path.collect())
            || los(t_path.collect())
            || los(b_path.collect());

        los_r
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
        assert_eq!(super::part2(INPUT), 999);
    }
}
