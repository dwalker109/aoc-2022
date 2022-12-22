use std::ops::RangeInclusive;

use super::*;

#[allow(dead_code)]
pub enum CubeNet<const N: usize> {
    Type1,
    Type2,
}

impl<const N: usize> CubeNet<N> {
    const fn grid_def(&self) -> [(RangeInclusive<usize>, RangeInclusive<usize>); 6] {
        match self {
            CubeNet::Type1 => [
                (N * 2 + 1..=N * 3, 1..=N),
                (1..=N, N + 1..=N * 2),
                (N + 1..=N * 2, N + 1..=N * 2),
                (N * 2 + 1..=N * 3, N + 1..=N * 2),
                (N * 2 + 1..=N * 3, N * 2 + 1..=N * 3),
                (N * 3 + 1..=N * 4, N * 2 + 1..=N * 3),
            ],
            CubeNet::Type2 => [
                (N + 1..=N * 2, 1..=N),
                (N * 2 + 1..=N * 3, 1..=N),
                (N + 1..=N * 2, N + 1..=N * 2),
                (1..=N, N * 2 + 1..=N * 3),
                (N + 1..=N * 2, N * 2 + 1..=N * 3),
                (1..=N, N * 3 + 1..=N * 4),
            ],
        }
    }

    pub fn build_portals(&self, mn: &mut MonkeyNotes) {
        let grid_def = self.grid_def();
        let g = (0..6)
            .map(|i| CubeFace::build(&mn.map, &grid_def[i]))
            .collect::<Vec<_>>();

        let mut from: &CubeFace;
        let mut to: &CubeFace;

        match &self {
            CubeNet::Type1 => {
                // 0
                from = &g[0];

                to = &g[1];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(*to.x_range.end() - from.offset_x(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }

                to = &g[5];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(*to.x_range.end(), *to.y_range.end() - from.offset_y(xy)),
                            Dir::Left,
                        ),
                    );
                }

                to = &g[2];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(to.x_range.start() + from.offset_y(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }

                // 1
                from = &g[1];

                to = &g[0];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(to.x_range.end() - from.offset_x(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }

                to = &g[4];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(to.x_range.end() - from.offset_x(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                to = &g[5];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(to.x_range.end() - from.offset_y(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                // 2
                from = &g[2];

                to = &g[0];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(*to.x_range.start(), to.y_range.start() + from.offset_x(xy)),
                            Dir::Right,
                        ),
                    );
                }

                to = &g[4];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(to.x_range.end() - from.offset_x(xy), *to.y_range.start()),
                            Dir::Right,
                        ),
                    );
                }

                // 3
                from = &g[3];

                to = &g[5];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(to.x_range.end() - from.offset_y(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }

                // 4
                from = &g[4];

                to = &g[2];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(to.x_range.end() - from.offset_y(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                to = &g[1];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(to.x_range.end() - from.offset_x(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                // 5
                from = &g[5];

                to = &g[3];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(*to.x_range.end(), to.y_range.end() - from.offset_x(xy)),
                            Dir::Left,
                        ),
                    );
                }

                to = &g[0];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(*to.x_range.end(), to.y_range.end() - from.offset_y(xy)),
                            Dir::Left,
                        ),
                    );
                }

                to = &g[1];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(*to.x_range.start(), to.y_range.end() - from.offset_x(xy)),
                            Dir::Right,
                        ),
                    );
                }
            }
            CubeNet::Type2 => {
                // 0
                from = &g[0];

                to = &g[5];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(*to.x_range.start(), *to.y_range.start() + from.offset_x(xy)),
                            Dir::Right,
                        ),
                    );
                }

                to = &g[3];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(*to.x_range.start(), to.y_range.end() - from.offset_y(xy)),
                            Dir::Right,
                        ),
                    );
                }

                // 1
                from = &g[1];

                to = &g[5];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(to.x_range.start() + from.offset_x(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                to = &g[4];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(*to.x_range.end(), *to.y_range.end() - from.offset_y(xy)),
                            Dir::Left,
                        ),
                    );
                }

                to = &g[2];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(*to.x_range.end(), *to.y_range.start() + from.offset_x(xy)),
                            Dir::Left,
                        ),
                    );
                }

                // 2
                from = &g[2];

                to = &g[1];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(*to.x_range.start() + from.offset_y(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                to = &g[3];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(*to.x_range.start() + from.offset_y(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }

                // 3
                from = &g[3];

                to = &g[2];
                for xy in &from.u {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Up)),
                        (
                            Xy::new(*to.x_range.start(), *to.y_range.start() + from.offset_x(xy)),
                            Dir::Right,
                        ),
                    );
                }

                to = &g[0];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(*to.x_range.start(), *to.y_range.end() - from.offset_y(xy)),
                            Dir::Right,
                        ),
                    );
                }

                // 4
                from = &g[4];

                to = &g[1];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(*to.x_range.end(), *to.y_range.end() - from.offset_y(xy)),
                            Dir::Left,
                        ),
                    );
                }

                to = &g[5];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(*to.x_range.end(), *to.y_range.start() + from.offset_x(xy)),
                            Dir::Left,
                        ),
                    );
                }

                // 5
                from = &g[5];

                to = &g[4];
                for xy in &from.r {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Right)),
                        (
                            Xy::new(*to.x_range.start() + from.offset_y(xy), *to.y_range.end()),
                            Dir::Up,
                        ),
                    );
                }

                to = &g[1];
                for xy in &from.d {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Down)),
                        (
                            Xy::new(*to.x_range.start() + from.offset_x(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }

                to = &g[0];
                for xy in &from.l {
                    mn.portals.insert(
                        (**xy, xy.adj(Dir::Left)),
                        (
                            Xy::new(*to.x_range.start() + from.offset_y(xy), *to.y_range.start()),
                            Dir::Down,
                        ),
                    );
                }
            }
        }
    }
}

struct CubeFace<'a> {
    u: Vec<&'a Xy>,
    r: Vec<&'a Xy>,
    d: Vec<&'a Xy>,
    l: Vec<&'a Xy>,
    x_range: &'a RangeInclusive<usize>,
    y_range: &'a RangeInclusive<usize>,
}

impl<'a> CubeFace<'a> {
    fn build(
        all: &'a HashMap<Xy, Tile>,
        (x_range, y_range): &'a (RangeInclusive<usize>, RangeInclusive<usize>),
    ) -> CubeFace<'a> {
        let members = all
            .keys()
            .filter(|xy| x_range.contains(xy.x()) && y_range.contains(xy.y()))
            .collect::<Vec<_>>();

        let u = members
            .iter()
            .filter(|xy| xy.y() == y_range.start())
            .copied()
            .collect::<Vec<_>>();

        let r = members
            .iter()
            .filter(|xy| xy.x() == x_range.end())
            .copied()
            .collect::<Vec<_>>();

        let d = members
            .iter()
            .filter(|xy| xy.y() == y_range.end())
            .copied()
            .collect::<Vec<_>>();

        let l = members
            .iter()
            .filter(|xy| xy.x() == x_range.start())
            .copied()
            .collect::<Vec<_>>();

        Self {
            u,
            r,
            d,
            l,
            x_range,
            y_range,
        }
    }

    fn offset_x(&self, xy: &Xy) -> usize {
        xy.x() - self.x_range.start()
    }

    fn offset_y(&self, xy: &Xy) -> usize {
        xy.y() - self.y_range.start()
    }
}
