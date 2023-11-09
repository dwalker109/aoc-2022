#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Xy(pub usize, pub usize);

impl Xy {
    pub fn adj(&self) -> [Self; 5] {
        [
            *self,
            Xy(self.0, self.1.saturating_sub(1)),
            Xy(self.0, self.1 + 1),
            Xy(self.0.saturating_sub(1), self.1),
            Xy(self.0 + 1, self.1),
        ]
    }
}
