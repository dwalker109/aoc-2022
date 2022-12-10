use std::fmt::Display;

pub(crate) struct Screen(pub(crate) [bool; 240]);

impl Default for Screen {
    fn default() -> Self {
        Self([false; 240])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(246);
        out.push('\n');
        for row in self.0.chunks_exact(40) {
            for c in row {
                out.push(match c {
                    true => '#',
                    false => '.',
                });
            }
            out.push('\n');
        }

        write!(f, "{out}")
    }
}
