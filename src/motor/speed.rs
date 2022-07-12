use derive_more::*;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Constructor, From,
)]
pub struct PerSecond<Units> {
    pub units: Units,
}

impl<Units> PerSecond<Units> {
    pub fn units(self) -> Units {
        self.units
    }
}
