use super::{StopAction, TachoMotorPositionUnit, TachoMotorSpeedUnit};

pub trait Rotate {
    fn rotate(
        &mut self,
        speed: impl TachoMotorSpeedUnit,
        shift: impl TachoMotorPositionUnit,
        stop_action: StopAction,
    ) -> anyhow::Result<()>;
}
