use super::{Degrees, TachoCounts};
use crate::percentage::{Percentage, SignedPercentage};

pub trait TachoMotorSpeedUnit {
    fn tacho_counts(
        self,
        count_per_rot: TachoCounts,
        max_speed: TachoCounts,
    ) -> TachoCounts;
}

impl TachoMotorSpeedUnit for TachoCounts {
    fn tacho_counts(self, _: TachoCounts, _: TachoCounts) -> TachoCounts {
        self
    }
}

impl TachoMotorSpeedUnit for SignedPercentage {
    fn tacho_counts(
        self,
        _: TachoCounts,
        max_speed: TachoCounts,
    ) -> TachoCounts {
        TachoCounts::new((max_speed.value() as f32 * self.to_fraction()) as i32)
    }
}

impl TachoMotorSpeedUnit for Percentage {
    fn tacho_counts(
        self,
        _: TachoCounts,
        max_speed: TachoCounts,
    ) -> TachoCounts {
        TachoCounts::new((max_speed.value() as f32 * self.to_fraction()) as i32)
    }
}

impl<T: Into<Degrees>> TachoMotorSpeedUnit for T {
    fn tacho_counts(
        self,
        count_per_rot: TachoCounts,
        _: TachoCounts,
    ) -> TachoCounts {
        let degrees = self.into();
        TachoCounts::new(degrees.value() * count_per_rot.value() / 360)
    }
}
