use std::ops::{Add, Sub};

use chamber::{jetstreams, Chamber};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> usize {
    let mut ch = Chamber::default();
    let mut r = rock::generator();
    let mut j = jetstreams(input);

    while ch.num_blocks() < 2022 {
        ch.add(r.next().unwrap());
        ch.run(&mut j);
    }

    usize::try_from(ch.height()).unwrap()
}

fn part2(input: &'static str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3068);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Xy(isize, isize);

impl Xy {
    pub fn x(&self) -> isize {
        self.0
    }

    pub fn y(&self) -> isize {
        self.1
    }
}

impl Add for Xy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Xy(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Xy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Xy(self.0 - rhs.0, self.1 - rhs.1)
    }
}

mod chamber {
    use std::collections::HashMap;

    use crate::{
        rock::{Rock, M},
        Xy,
    };

    const MIN_X: isize = 0;
    const MAX_X: isize = 6;
    const MIN_Y: isize = 0;

    #[derive(Default, Debug)]
    pub struct Chamber {
        settled: HashMap<Xy, M>,
        active: Option<(Rock, Xy, Vec<(Xy, M)>)>,
        height: isize,
        blocks: usize,
    }

    impl Chamber {
        pub fn num_rocks(&self) -> usize {
            self.settled.len()
        }

        pub fn height(&self) -> isize {
            self.height
        }

        pub fn num_blocks(&self) -> usize {
            self.blocks
        }

        pub fn add(&mut self, rock: Rock) {
            let (rock, mut positions) = match self.active.take() {
                Some((_, _, positions)) => (rock, positions),
                None => (rock, Vec::with_capacity(9)),
            };

            let origin = Xy(2, self.height + 3);
            rock.fill_template(&mut positions, &origin);

            self.active = Some((rock, origin, positions));
        }

        pub fn run(&mut self, jetstreams: &mut impl Iterator<Item = Jet>) {
            let (rock, origin, mut positions) = self.active.take().unwrap();

            let can_move = |(xy, m): &(Xy, M)| {
                if matches!(m, M::Space) {
                    return true;
                }

                if xy.x() < MIN_X || xy.x() > MAX_X || xy.y() < MIN_Y {
                    return false;
                }

                match self.settled.get(xy) {
                    Some(M::Rock) => false,
                    Some(M::Space) | None => true,
                }
            };

            loop {
                match jetstreams.next().unwrap() {
                    Jet::L => {
                        let next = positions
                            .iter()
                            .map(|(xy, m)| (*xy - Xy(1, 0), *m))
                            .collect::<Vec<_>>();
                        if next.iter().all(can_move) {
                            positions = next;
                        }
                    }
                    Jet::R => {
                        let next = positions
                            .iter()
                            .map(|(xy, m)| (*xy + Xy(1, 0), *m))
                            .collect::<Vec<_>>();
                        if next.iter().all(can_move) {
                            positions = next;
                        }
                    }
                }

                let next = positions
                    .iter()
                    .map(|(xy, m)| (*xy - Xy(0, 1), *m))
                    .collect::<Vec<_>>();
                if next.iter().all(can_move) {
                    positions = next;
                } else {
                    break;
                }
            }

            self.blocks += 1;

            self.height = std::cmp::max(
                self.height,
                positions.iter().max_by_key(|p| p.0.y()).unwrap().0.y() + 1,
            );

            self.settled
                .extend(positions.into_iter().filter(|(_, m)| matches!(m, M::Rock)));
        }
    }

    #[derive(Clone, Copy)]
    pub enum Jet {
        L,
        R,
    }

    impl From<char> for Jet {
        fn from(c: char) -> Self {
            match c {
                '<' => Self::L,
                '>' => Self::R,
                _ => panic!("invalid dir ({c})"),
            }
        }
    }

    pub fn jetstreams(input: &'static str) -> impl Iterator<Item = Jet> {
        input.trim().chars().map(Jet::from).into_iter().cycle()
    }
}

mod rock {
    use super::Xy;

    pub fn generator() -> impl Iterator<Item = Rock> {
        [
            Rock::make(1),
            Rock::make(2),
            Rock::make(3),
            Rock::make(4),
            Rock::make(5),
        ]
        .into_iter()
        .cycle()
    }

    #[derive(Clone, Copy, Debug)]
    pub enum M {
        Rock,
        Space,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum Rock {
        T1([(Xy, M); 4]),
        T2([(Xy, M); 9]),
        T3([(Xy, M); 9]),
        T4([(Xy, M); 4]),
        T5([(Xy, M); 4]),
    }

    impl Rock {
        fn make(id: u8) -> Self {
            match id {
                1 => Self::T1([
                    (Xy(0, 0), M::Rock),
                    (Xy(1, 0), M::Rock),
                    (Xy(2, 0), M::Rock),
                    (Xy(3, 0), M::Rock),
                ]),
                2 => Self::T2([
                    (Xy(0, 0), M::Space),
                    (Xy(1, 0), M::Rock),
                    (Xy(2, 0), M::Space),
                    (Xy(0, 1), M::Rock),
                    (Xy(1, 1), M::Rock),
                    (Xy(2, 1), M::Rock),
                    (Xy(0, 2), M::Space),
                    (Xy(1, 2), M::Rock),
                    (Xy(2, 2), M::Space),
                ]),
                3 => Self::T3([
                    (Xy(0, 0), M::Rock),
                    (Xy(1, 0), M::Rock),
                    (Xy(2, 0), M::Rock),
                    (Xy(0, 1), M::Space),
                    (Xy(1, 1), M::Space),
                    (Xy(2, 1), M::Rock),
                    (Xy(0, 2), M::Space),
                    (Xy(1, 2), M::Space),
                    (Xy(2, 2), M::Rock),
                ]),
                4 => Self::T4([
                    (Xy(0, 0), M::Rock),
                    (Xy(0, 1), M::Rock),
                    (Xy(0, 2), M::Rock),
                    (Xy(0, 3), M::Rock),
                ]),
                5 => Self::T5([
                    (Xy(0, 0), M::Rock),
                    (Xy(1, 0), M::Rock),
                    (Xy(0, 1), M::Rock),
                    (Xy(1, 1), M::Rock),
                ]),
                _ => panic!("invalid rock kind"),
            }
        }

        pub fn fill_template(&self, template: &mut Vec<(Xy, M)>, origin_xy: &Xy) {
            let (n, mut p) = match self {
                Rock::T1(p) => (p.len(), p.iter()),
                Rock::T2(p) => (p.len(), p.iter()),
                Rock::T3(p) => (p.len(), p.iter()),
                Rock::T4(p) => (p.len(), p.iter()),
                Rock::T5(p) => (p.len(), p.iter()),
            };

            template.clear();
            template.resize(n, (Xy::default(), M::Space));

            template
                .iter_mut()
                .take(n)
                .for_each(|(template_xy, template_matter)| {
                    let (offset, curr_matter) = p.next().unwrap();
                    *template_xy = *origin_xy + *offset;
                    *template_matter = *curr_matter;
                });
        }
    }
}
