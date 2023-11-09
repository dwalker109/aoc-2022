use crate::coords::Xy;
use crate::matter::{Item, Storm};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct Valley(HashMap<Xy, Item>, Vec<Storm>);

impl Valley {
    pub fn next_storms(&mut self) -> HashSet<Xy> {
        self.1.iter_mut().map(|s| s.r#move()).collect()
    }

    pub fn entrance_pos(&self) -> Xy {
        *self
            .0
            .iter()
            .find(|(_xy, item)| matches!(item, Item::Entrance))
            .unwrap()
            .0
    }

    pub fn exit_pos(&self) -> Xy {
        *self
            .0
            .iter()
            .find(|(_xy, item)| matches!(item, Item::Exit))
            .unwrap()
            .0
    }

    pub fn pos_blocked(&self, xy: &Xy) -> bool {
        match self.0.get(xy) {
            None => true,
            Some(Item::Blocked) => true,
            Some(_) => false,
        }
    }

    pub fn nav(&mut self, from: &Xy, to: &Xy) -> usize {
        let mut from = vec![*from].into_iter().collect::<HashSet<_>>();

        for i in 1_usize.. {
            let storms = self.next_storms();

            let mut next = from
                .iter()
                .flat_map(|p| p.adj())
                .filter(|xy| !self.pos_blocked(xy) && !storms.contains(xy))
                .collect::<HashSet<_>>();

            match next.contains(to) {
                true => return i,
                false => std::mem::swap(&mut from, &mut next),
            };
        }

        unreachable!()
    }
}

impl From<&str> for Valley {
    fn from(raw: &str) -> Self {
        let max_x = raw.lines().next().unwrap().chars().count() - 1;
        let max_y = raw.lines().count() - 1;

        let storms = Rc::new(RefCell::new(Vec::new()));

        let cells: HashMap<Xy, Item> = raw
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                let s = Rc::clone(&storms);
                l.chars().enumerate().map(move |(x, c)| {
                    let xy = Xy(x, y);

                    let item = Item::from((c, y == 0, y == max_y));

                    if matches!(c, '^' | 'v' | '<' | '>') {
                        s.borrow_mut().push(Storm::from((xy, c, max_x, max_y)))
                    }

                    (xy, item)
                })
            })
            .collect();

        let storms = storms.take();

        Self(cells, storms)
    }
}
