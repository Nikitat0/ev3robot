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
    fn stop<StopAction: ToString>(
        &mut self,
        stop_action: StopAction,
    ) -> anyhow::Result<()> {
        self.stop_action.set_value(stop_action)?;
        self.command.set_value("stop")?;
        Ok(())
    }
}

macro_rules! tacho_motor_stop_action {
    ($stop_action:ty) => {
        impl Stop<$stop_action> for TachoMotor {
            fn stop(
                &mut self,
                stop_action: $stop_action,
            ) -> anyhow::Result<()> {
                self.stop(stop_action)
            }
        }
    };
}

tacho_motor_stop_action!(Brake);
tacho_motor_stop_action!(Coast);
tacho_motor_stop_action!(Hold);
