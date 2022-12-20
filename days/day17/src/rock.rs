use super::Xy;

pub fn generator() -> impl Iterator<Item = Rock> {
    [
        Rock::make(1),
        Rock::make(2),
        Rock::make(3),
        Rock::make(4),
        Rock::make(5),
    ]
    .into_iter()
    .cycle()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rock {
    T1([Xy; 4]),
    T2([Xy; 5]),
    T3([Xy; 5]),
    T4([Xy; 4]),
    T5([Xy; 4]),
}

impl Rock {
    fn make(id: u8) -> Self {
        match id {
            1 => Self::T1([Xy(0, 0), Xy(1, 0), Xy(2, 0), Xy(3, 0)]),
            2 => Self::T2([Xy(1, 0), Xy(0, 1), Xy(1, 1), Xy(2, 1), Xy(1, 2)]),
            3 => Self::T3([Xy(0, 0), Xy(1, 0), Xy(2, 0), Xy(2, 1), Xy(2, 2)]),
            4 => Self::T4([Xy(0, 0), Xy(0, 1), Xy(0, 2), Xy(0, 3)]),
            5 => Self::T5([Xy(0, 0), Xy(1, 0), Xy(0, 1), Xy(1, 1)]),
            _ => panic!("invalid rock kind"),
        }
    }

    pub fn fill_template(&self, template: &mut Vec<Xy>, origin_xy: &Xy) {
        let (n, mut p) = match self {
            Rock::T1(p) => (p.len(), p.iter()),
            Rock::T2(p) => (p.len(), p.iter()),
            Rock::T3(p) => (p.len(), p.iter()),
            Rock::T4(p) => (p.len(), p.iter()),
            Rock::T5(p) => (p.len(), p.iter()),
        };

        template.clear();
        template.resize(n, Xy::default());

        template.iter_mut().take(n).for_each(|template_xy| {
            let offset = p.next().unwrap();
            *template_xy = *origin_xy + *offset;
        });
    }
}
