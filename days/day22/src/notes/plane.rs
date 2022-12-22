use std::collections::HashSet;

use super::*;

pub fn build_portals(mn: &mut MonkeyNotes) {
    let ys = mn
        .map
        .keys()
        .map(|xy| xy.y())
        .copied()
        .collect::<HashSet<_>>();
    let xs = mn
        .map
        .keys()
        .map(|xy| xy.x())
        .copied()
        .collect::<HashSet<_>>();

    let y_axis = ys
        .iter()
        .flat_map(|y| {
            let &lowest = mn.map.keys().filter(|xy| xy.y() == y).min().unwrap();
            let &highest = mn.map.keys().filter(|xy| xy.y() == y).max().unwrap();

            [
                ((lowest, lowest.adj(Dir::Left)), (highest, Dir::Left)),
                ((highest, highest.adj(Dir::Right)), (lowest, Dir::Right)),
            ]
        })
        .collect::<Vec<_>>();

    let x_axis = xs
        .iter()
        .flat_map(|x| {
            let &lowest = mn.map.keys().filter(|xy| xy.x() == x).min().unwrap();
            let &highest = mn.map.keys().filter(|xy| xy.x() == x).max().unwrap();

            [
                ((lowest, lowest.adj(Dir::Up)), (highest, Dir::Up)),
                ((highest, highest.adj(Dir::Down)), (lowest, Dir::Down)),
            ]
        })
        .collect::<Vec<_>>();

    mn.portals
        .extend(y_axis.into_iter().chain(x_axis.into_iter()));
}
