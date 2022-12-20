use std::{cell::RefCell, rc::Rc};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> isize {
    let mut l = NumList::from(input);
    l.mix();

    let a = l.n_after_0(1000);
    let b = l.n_after_0(2000);
    let c = l.n_after_0(3000);

    dbg!(a, b, c);

    a + b + c
}

fn part2(input: &'static str) -> isize {
    todo!()
}

#[derive(Debug, Default)]
struct Num {
    val: isize,
    prev: Option<Rc<RefCell<Num>>>,
    next: Option<Rc<RefCell<Num>>>,
}

impl From<&str> for Num {
    fn from(value: &str) -> Self {
        Self {
            val: value.parse().unwrap(),
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct NumList(Vec<Rc<RefCell<Num>>>);

impl NumList {
    pub fn mix(&mut self) {
        for i in 0..self.0.len() {
            // Num to move
            let src_ref = self.0.get(i).unwrap();

            // Do nothing for 0
            if (*src_ref).borrow().val == 0 {
                continue;
            }

            if [6, 7, 8, 9].contains(&(*src_ref).borrow().val) {
                continue;
            }

            // Remove src from the order by connecting its prev and next to each other
            let src_lhs = Rc::clone((*src_ref).borrow().prev.as_ref().unwrap());
            let src_rhs = Rc::clone((*src_ref).borrow().next.as_ref().unwrap());
            (*src_lhs).borrow_mut().next.replace(Rc::clone(&src_rhs));
            (*src_rhs).borrow_mut().prev.replace(Rc::clone(&src_lhs));

            // Init vars we will use for the prev and next either side of the moved Num
            let mut dest_lhs = Rc::new(RefCell::new(Num::default()));
            let mut dest_rhs = Rc::new(RefCell::new(Num::default()));

            // Going left...
            if (*src_ref).borrow().val.is_negative() {
                dest_rhs = Rc::clone((*src_ref).borrow().prev.as_ref().unwrap());

                for _ in 1..(*src_ref).borrow().val.abs() {
                    let tmp = Rc::clone(dest_rhs.borrow().prev.as_ref().unwrap());
                    dest_rhs = tmp;
                }

                dest_lhs = Rc::clone(dest_rhs.borrow().prev.as_ref().unwrap());
            }

            // ... or right
            if (*src_ref).borrow().val.is_positive() {
                dest_lhs = Rc::clone((*src_ref).borrow().next.as_ref().unwrap());

                for _ in 1..(*src_ref).borrow().val {
                    let tmp = Rc::clone(dest_lhs.borrow().next.as_ref().unwrap());
                    dest_lhs = tmp;
                }

                dest_rhs = Rc::clone(dest_lhs.borrow().next.as_ref().unwrap());
            }

            // Connect either side of moved src in
            dest_lhs.borrow_mut().next.replace(Rc::clone(src_ref));
            dest_rhs.borrow_mut().prev.replace(Rc::clone(src_ref));

            // Connect moved src either side out
            (*src_ref).borrow_mut().prev.replace(Rc::clone(&dest_lhs));
            (*src_ref).borrow_mut().next.replace(Rc::clone(&dest_rhs));
        }
    }

    pub fn n_after_0(&self, n: usize) -> isize {
        let mut node = self.0.iter().find(|n| n.borrow().val == 0).unwrap().clone();
        for _ in 0..n {
            let tmp = node.borrow().next.as_ref().unwrap().clone();
            node = tmp;
        }

        let n = node.borrow().val;

        n
    }
}

impl From<&str> for NumList {
    fn from(value: &str) -> Self {
        let nodes = value
            .lines()
            .map(|n| Rc::new(RefCell::new(Num::from(n))))
            .collect::<Vec<_>>();

        let l = nodes.len();

        for i in 0..l {
            let prev_i = (i + l - 1) % l;
            let next_i = (i + 1) % l;

            let mut curr = (**nodes.get(i).unwrap()).borrow_mut();
            let prev = Rc::clone(nodes.get(prev_i).unwrap());
            let next = Rc::clone(nodes.get(next_i).unwrap());

            curr.prev.replace(prev);
            curr.next.replace(next);
        }

        Self(nodes)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3);
    }
}
