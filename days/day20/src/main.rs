use std::{cell::RefCell, ops::Rem, rc::Rc};

static INPUT: &str = include_str!("../input");

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> isize {
    let mut l = NumList::from(input);

    l.mix(1);
    l.grove_coordinatees().iter().sum()
}

fn part2(input: &'static str) -> isize {
    let mut l = NumList::from(input);

    l.decrypt();
    l.mix(10);
    l.grove_coordinatees().iter().sum()
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
    pub fn mix(&mut self, times: usize) {
        (0..times).for_each(|_| (0..self.0.len()).for_each(|i| self.do_mix(i)));
    }

    fn do_mix(&mut self, i: usize) {
        // Num to move
        let src_ref = self.0.get(i).unwrap();

        // How many moves to make, ignoring cycles via remainder (-1 since we
        // have to exclude the item being moved from this calculation (╯°□°)╯︵ ┻━┻)
        let distance = (*src_ref).borrow().val.rem(self.0.len() as isize - 1);

        // Do nothing for 0
        if distance == 0 {
            return;
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
        if distance.is_negative() {
            dest_rhs = Rc::clone((*src_ref).borrow().prev.as_ref().unwrap());

            for _ in 1..distance.abs() {
                let tmp = Rc::clone(dest_rhs.borrow().prev.as_ref().unwrap());
                dest_rhs = tmp;
            }

            dest_lhs = Rc::clone(dest_rhs.borrow().prev.as_ref().unwrap());
        }

        // ... or right
        if distance.is_positive() {
            dest_lhs = Rc::clone((*src_ref).borrow().next.as_ref().unwrap());

            for _ in 1..distance {
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

    pub fn grove_coordinatees(&self) -> Vec<isize> {
        let mut results = Vec::with_capacity(3);

        let mut node = self.0.iter().find(|n| n.borrow().val == 0).unwrap().clone();
        for i in 1..=3000 {
            let curr = node.borrow().next.as_ref().unwrap().clone();
            if i % 1000 == 0 {
                results.push(curr.borrow().val);
            }
            node = curr;
        }

        results
    }

    pub fn decrypt(&mut self) {
        for n in self.0.iter_mut() {
            (*n).borrow_mut().val *= 811589153;
        }
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

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1623178306);
    }
}
