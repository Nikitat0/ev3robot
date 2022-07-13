mod command;
mod state;

pub use command::*;
pub use state::*;

use super::*;
use crate::device::{
    ReadOnlyAttributeFile, ReadWriteAttributeFile, WriteOnlyAttributeFile,
};

#[allow(unused)]
#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "tacho-motor")]
pub struct TachoMotor {
    command: WriteOnlyAttributeFile,
    count_per_rot: u32,
    duty_cycle: ReadOnlyAttributeFile,
    duty_cycle_sp: ReadWriteAttributeFile,
    polarity: ReadWriteAttributeFile,
    position: ReadWriteAttributeFile,
    position_sp: ReadWriteAttributeFile,
    max_speed: u32,
    state: ReadOnlyAttributeFile,
    speed: ReadOnlyAttributeFile,
    speed_sp: ReadWriteAttributeFile,
    stop_action: ReadWriteAttributeFile,
}

impl TachoMotor {
    fn set_stop_action<S: TachoMotorStopAction>(
        &mut self,
        stop_action: S,
    ) -> anyhow::Result<()> {
        Ok(self.stop_action.set_value(stop_action)?)
    }
}

impl<S: TachoMotorStopAction> Stop<S> for TachoMotor {
    fn stop(&mut self, stop_action: S) -> anyhow::Result<()> {
        self.set_stop_action(stop_action)?;
        self.command.set_value("stop")?;
        Ok(())
    }
}

trait TachoMotorStopAction: ToString {}

impl TachoMotorStopAction for Brake {}
impl TachoMotorStopAction for Coast {}
impl TachoMotorStopAction for Hold {}
