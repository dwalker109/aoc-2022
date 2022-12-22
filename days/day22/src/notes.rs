use crate::space::{Dir, Step, Tile, Xy};
use std::collections::HashMap;

pub(crate) mod cube;
pub(crate) mod plane;

#[derive(Debug)]
pub struct MonkeyNotes {
    map: HashMap<Xy, Tile>,
    portals: HashMap<(Xy, Xy), (Xy, Dir)>,
    pos: (Xy, Dir),
    instr: Vec<Step>,
}

impl From<&str> for MonkeyNotes {
    fn from(value: &str) -> Self {
        let (map, pos, instr) = parse(value);

        Self {
            map,
            portals: HashMap::new(),
            pos,
            instr,
        }
    }
}

impl MonkeyNotes {
    pub fn nav(&mut self) {
        for step in self.instr.iter() {
            match step {
                Step::Fwd(mut n) => {
                    while n > 0 {
                        let next_xy = self.pos.0.adj(self.pos.1);

                        match self.map.get(&next_xy) {
                            Some(next_tile) => match next_tile {
                                Tile::Open => {
                                    self.pos.0 = next_xy;
                                    n -= 1;
                                }
                                Tile::Wall => break,
                            },
                            None => {
                                let (next_xy, next_dir) =
                                    self.portals.get(&(self.pos.0, next_xy)).unwrap();
                                match self.map.get(next_xy).unwrap() {
                                    Tile::Open => {
                                        self.pos = (*next_xy, *next_dir);
                                        n -= 1;
                                    }
                                    Tile::Wall => break,
                                }
                            }
                        }
                    }
                }
                Step::Trn(dir) => {
                    self.pos.1 = self.pos.1.turn(dir);
                }
            }
        }
    }

    pub fn password(&self) -> usize {
        (1000 * (self.pos.0.y())) + (4 * (self.pos.0.x())) + self.pos.1 as usize
    }
}

fn parse(input: &str) -> (HashMap<Xy, Tile>, (Xy, Dir), Vec<Step>) {
    let (board, steps) = input.split_once("\n\n").unwrap();

    let map = board
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                Tile::try_from(c).ok().map(|t| {
                    // 1-index
                    (Xy(x + 1, y + 1), t)
                })
            })
        })
        .collect::<HashMap<_, _>>();

    let pos = map
        .iter()
        .filter(|(xy, tile)| xy.y() == &1 && tile.is_open())
        .min_by_key(|(xy, _)| *xy)
        .map(|(xy, _)| (*xy, Dir::Right))
        .unwrap();

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

    (map, pos, instr)
}
