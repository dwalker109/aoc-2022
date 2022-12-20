use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::{rock::Rock, Xy};

const MIN_X: isize = 0;
const MAX_X: isize = 6;
const MIN_Y: isize = 0;

#[derive(Default, Debug)]
pub struct Chamber {
    settled: HashSet<Xy>,
    active: Option<(Rock, Vec<Xy>)>,
    height: isize,
    skipped: isize,
    blocks: usize,
    cycle: HashMap<(Rock, Jet, [usize; 8]), (usize, isize)>,
    rolling_height_delta: [usize; 8],
}

impl Chamber {
    pub fn height(&self) -> isize {
        self.height + self.skipped
    }

    pub fn num_blocks(&self) -> usize {
        self.blocks
    }

    pub fn add(&mut self, rock: Rock) {
        let (rock, mut positions) = match self.active.take() {
            Some((_, positions)) => (rock, positions),
            None => (rock, Vec::with_capacity(9)),
        };

        let origin = Xy(2, self.height + 3);
        rock.fill_template(&mut positions, &origin);

        self.active = Some((rock, positions));
    }

    pub fn run<const N: usize>(
        &mut self,
        rocks: &mut impl Iterator<Item = Rock>,
        jetstreams: &mut impl Iterator<Item = Jet>,
    ) {
        let mut jetstreams = jetstreams.peekable();

        while self.num_blocks() < N {
            self.add(rocks.next().unwrap());
            let (rock, mut positions) = self.active.take().unwrap();

            if self.skipped == 0 {
                let key = (rock, *jetstreams.peek().unwrap(), self.rolling_height_delta);
                match self.cycle.entry(key) {
                    Entry::Occupied(e) => {
                        let (prev_blocks, prev_height) = e.get();

                        let cycle_blocks = self.blocks - prev_blocks;
                        let cycle_height = self.height - prev_height;
                        let repeats = (N - self.blocks) / cycle_blocks;

                        self.blocks += cycle_blocks * repeats;
                        self.skipped = cycle_height * repeats as isize;
                    }
                    Entry::Vacant(e) => {
                        e.insert((self.blocks, self.height));
                    }
                }
            }

            let can_move = |xy: &Xy| {
                if xy.x() < MIN_X || xy.x() > MAX_X || xy.y() < MIN_Y {
                    return false;
                }

                !self.settled.contains(xy)
            };

            loop {
                let jetstream = jetstreams.next().unwrap();

                match jetstream {
                    Jet::L(_) => {
                        let next = positions
                            .iter()
                            .map(|xy| *xy - Xy(1, 0))
                            .collect::<Vec<_>>();
                        if next.iter().all(can_move) {
                            positions = next;
                        }
                    }
                    Jet::R(_) => {
                        let next = positions
                            .iter()
                            .map(|xy| *xy + Xy(1, 0))
                            .collect::<Vec<_>>();
                        if next.iter().all(can_move) {
                            positions = next;
                        }
                    }
                }

                let next = positions
                    .iter()
                    .map(|xy| *xy - Xy(0, 1))
                    .collect::<Vec<_>>();
                if next.iter().all(can_move) {
                    positions = next;
                } else {
                    break;
                }
            }

            let next_height = self
                .height
                .max(positions.iter().max_by_key(|p| p.y()).unwrap().y() + 1);

            self.rolling_height_delta[self.blocks % 8] = self.height.abs_diff(next_height);
            self.height = next_height;
            self.blocks += 1;
            self.settled.extend(positions.into_iter());
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Jet {
    L(usize),
    R(usize),
}

impl From<(usize, char)> for Jet {
    fn from((n, c): (usize, char)) -> Self {
        match c {
            '<' => Self::L(n),
            '>' => Self::R(n),
            _ => panic!("invalid dir ({c})"),
        }
    }
}

pub fn jetstreams(input: &'static str) -> impl Iterator<Item = Jet> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(Jet::from)
        .into_iter()
        .cycle()
}
