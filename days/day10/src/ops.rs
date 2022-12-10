pub(crate) struct Ops(pub(crate) Vec<isize>);

impl From<&str> for Ops {
    fn from(raw: &str) -> Self {
        let mut fns = Vec::new();
        for l in raw.lines() {
            let mut w = l.splitn(2, ' ');
            let a = w.next().unwrap();
            let b = w.next().unwrap_or_default();

            match a {
                "noop" => {
                    fns.push(0);
                }
                "addx" => {
                    let b = b.parse::<isize>().unwrap();
                    fns.push(0);
                    fns.push(b);
                }
                _ => panic!(),
            }
        }

        Self(fns)
    }
}
