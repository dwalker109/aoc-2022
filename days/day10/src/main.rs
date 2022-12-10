static INPUT: &str = include_str!("../input");

mod ops;
mod screen;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT));
}

fn part1(input: &'static str) -> String {
    let mut x = 1;
    let mut cycle = 1;
    let mut ss = Vec::new();

    for op in ops::Ops::from(input).0 {
        if (cycle - 20) % 40 == 0 {
            ss.push(cycle * x);
        }

        cycle += 1;
        x += op;
    }

    format!("{}", ss.iter().sum::<isize>())
}

fn part2(input: &'static str) -> String {
    let mut cycle = 1;
    let mut x = 1;
    let mut screen = screen::Screen::default();

    for op in ops::Ops::from(input).0 {
        let row_mod = match cycle {
            1..=40 => 0,
            41..=80 => 40,
            81..=120 => 80,
            121..=160 => 120,
            161..=200 => 160,
            201..=240 => 200,
            _ => panic!(),
        };

        let draw_pos = cycle - 1;
        let sprite = [x + row_mod - 1, x + row_mod, x + row_mod + 1];

        if sprite.contains(&draw_pos) {
            screen.0[usize::try_from(draw_pos).expect("is never negative")] = true;
        }

        cycle += 1;
        x += op;
    }

    format!("{screen}")
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), "13140");
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(INPUT),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}
