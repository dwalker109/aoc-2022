use std::collections::HashMap;

use crate::space::{Dir, Step, Tile, Xy};

pub struct MonkeyNotes {
    map: HashMap<Xy, Tile>,
    pos: (Xy, Dir),
    instr: Vec<Step>,
}

impl MonkeyNotes {
    pub fn nav(&mut self) {
        for step in self.instr.iter() {
            let (Xy(pos_x, pos_y), pos_dir) = &self.pos;

            let mut plane = match pos_dir {
                Dir::Up | Dir::Down => self
                    .map
                    .keys()
                    .filter(|Xy(x, _)| x == pos_x)
                    .collect::<Vec<_>>(),
                Dir::Left | Dir::Right => self
                    .map
                    .keys()
                    .filter(|Xy(_, y)| y == pos_y)
                    .collect::<Vec<_>>(),
            };

            plane.sort_unstable();
            if matches!(pos_dir, Dir::Left | Dir::Up) {
                plane.reverse();
            }

            match step {
                Step::Fwd(steps) => {
                    let init = plane.iter().position(|xy| xy == &&self.pos.0).unwrap();
                    for p in plane
                        .iter()
                        .cycle()
                        .skip(init + 1)
                        .take(*steps)
                        .take_while(|p| self.map.get(p).unwrap().is_open())
                    {
                        self.pos.0 = **p;
                    }
                }
                Step::Trn(rotation) => {
                    self.pos.1 = (self.pos.1).turn(rotation);
                }
            }
        }
    }

    pub fn password(&self) -> usize {
        (1000 * (self.pos.0 .1 + 1))
            + (4 * (self.pos.0 .0 + 1))
            + match self.pos.1 {
                Dir::Right => 0,
                Dir::Down => 1,
                Dir::Left => 2,
                Dir::Up => 3,
            }
    }
}

impl From<&str> for MonkeyNotes {
    /// Oh god. Probably the ugliest code I've written this year. I'm tired.
    fn from(value: &str) -> Self {
        let (board, steps) = value.split_once("\n\n").unwrap();

        let map = board
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| Tile::try_from(c).ok().map(|t| (Xy(x, y), t)))
            })
            .collect::<HashMap<_, _>>();

        let init_pos = *map.keys().filter(|Xy(_, y)| y == &0).min().unwrap();

        let mut instr = Vec::with_capacity(steps.len());
        let mut curr_num = String::new();
        for c in steps.chars().chain(['!'].iter().copied()) {
            if c.is_numeric() {
                curr_num.push(c);
                continue;
            }

            if !curr_num.is_empty() {
                instr.push(Step::Fwd(curr_num.parse().unwrap()));
                curr_num.clear();
            }

            match c {
                'L' => instr.push(Step::Trn(Dir::Left)),
                'R' => instr.push(Step::Trn(Dir::Right)),
                _ => break,
            };
        }

        Self {
            map,
            pos: (init_pos, Dir::Right),
            instr,
        }
    }
}
