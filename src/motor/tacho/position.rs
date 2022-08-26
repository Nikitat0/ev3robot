use super::{Degrees, TachoCounts};

pub trait TachoMotorPositionUnit: Clone {
    fn tacho_counts(self, count_per_rot: TachoCounts) -> TachoCounts;
}

impl TachoMotorPositionUnit for TachoCounts {
    fn tacho_counts(self, _: TachoCounts) -> TachoCounts {
        self
    }
}

impl<T: Into<Degrees> + Clone> TachoMotorPositionUnit for T {
    fn tacho_counts(self, count_per_rot: TachoCounts) -> TachoCounts {
        let degrees = self.into();
        TachoCounts::new(degrees.value() * count_per_rot.value() / 360)
    }
}
