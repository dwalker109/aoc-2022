use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    dbg!(input);

    let mut grove = Grove::from(input);

    for i in 0..10_usize {
        let mut remain = HashSet::with_capacity(grove.0.len());
        let mut moves = HashMap::<Xy, Vec<Xy>>::new();

        'item: for xy in grove.0.iter() {
            if !has_neighbours(&grove, xy) {
                remain.insert(*xy);
                continue 'item;
            }

            for rule in make_proposal_iter().skip(i.rem_euclid(4)).take(4) {
                if let Some(p) = rule(&grove, xy) {
                    (*moves.entry(p).or_default()).push(*xy);
                    continue 'item;
                }
            }

            remain.insert(*xy);
        }

        grove.update(remain, moves);
    }

    grove.calc_empty_tiles()
}

fn part2(input: &'static str) -> usize {
    todo!()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Xy(isize, isize);

impl Xy {
    fn n(&self) -> Self {
        Xy(self.0, self.1 - 1)
    }

    fn ne(&self) -> Self {
        Xy(self.0 + 1, self.1 - 1)
    }

    fn e(&self) -> Self {
        Xy(self.0 + 1, self.1)
    }

    fn se(&self) -> Self {
        Xy(self.0 + 1, self.1 + 1)
    }

    fn s(&self) -> Self {
        Xy(self.0, self.1 + 1)
    }

    fn sw(&self) -> Self {
        Xy(self.0 - 1, self.1 + 1)
    }

    fn w(&self) -> Self {
        Xy(self.0 - 1, self.1)
    }

    fn nw(&self) -> Self {
        Xy(self.0 - 1, self.1 - 1)
    }
}

#[derive(Debug)]
struct Grove(HashSet<Xy>);

impl From<&str> for Grove {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter(|(_, c)| *c == '#')
                        .map(move |(x, _)| Xy(x.try_into().unwrap(), y.try_into().unwrap()))
                })
                .collect(),
        )
    }
}

impl Grove {
    fn update(&mut self, r: HashSet<Xy>, p: HashMap<Xy, Vec<Xy>>) {
        self.0 = r;

        for (m, s) in p {
            match s.len() {
                1 => {
                    self.0.insert(m);
                }
                _ => {
                    self.0.extend(s.iter());
                }
            }
        }
    }

    fn calc_empty_tiles(&self) -> usize {
        let min_x = self.0.iter().min_by_key(|xy| xy.0).unwrap().0;
        let max_x = self.0.iter().max_by_key(|xy| xy.0).unwrap().0;
        let min_y = self.0.iter().min_by_key(|xy| xy.1).unwrap().1;
        let max_y = self.0.iter().max_by_key(|xy| xy.1).unwrap().1;

        let w = min_x.abs_diff(max_x) + 1;
        let h = min_y.abs_diff(max_y) + 1;

        (w * h) - self.0.len()
    }
}

fn all_unoccupied(g: &Grove, l: &[&Xy]) -> bool {
    !l.iter().any(|&xy| g.0.contains(xy))
}

fn has_neighbours(g: &Grove, xy: &Xy) -> bool {
    !all_unoccupied(
        g,
        &[
            &xy.n(),
            &xy.ne(),
            &xy.e(),
            &xy.se(),
            &xy.s(),
            &xy.sw(),
            &xy.w(),
            &xy.nw(),
        ],
    )
}

fn make_proposal_iter() -> Box<dyn Iterator<Item = fn(&Grove, &Xy) -> Option<Xy>>> {
    let fns = [
        |g: &Grove, xy: &Xy| all_unoccupied(g, &[&xy.n(), &xy.ne(), &xy.nw()]).then_some(xy.n()),
        |g: &Grove, xy: &Xy| all_unoccupied(g, &[&xy.s(), &xy.se(), &xy.sw()]).then_some(xy.s()),
        |g: &Grove, xy: &Xy| all_unoccupied(g, &[&xy.w(), &xy.nw(), &xy.sw()]).then_some(xy.w()),
        |g: &Grove, xy: &Xy| all_unoccupied(g, &[&xy.e(), &xy.ne(), &xy.se()]).then_some(xy.e()),
    ];

    Box::new(fns.into_iter().cycle())
}

mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 110);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 0);
    }
}
