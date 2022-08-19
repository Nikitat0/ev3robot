mod command;
mod state;
mod stop_action;
mod tacho_counts;

pub use command::*;
pub use state::*;
pub use stop_action::*;
pub use tacho_counts::*;

use super::Polarity;
use crate::device::{
    ReadOnlyAttributeFile, ReadWriteAttributeFile, WriteOnlyAttributeFile,
};
use crate::percentage::SignedPercentage;

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

pub trait TachoMotorInterface {
    fn command(&mut self, command: Command) -> anyhow::Result<()>;
    fn count_per_rot(&self) -> TachoCounts;
    fn duty_cycle(&mut self) -> anyhow::Result<SignedPercentage>;
    fn duty_cycle_sp(&mut self) -> anyhow::Result<SignedPercentage>;
    fn set_duty_cycle_sp(
        &mut self,
        value: SignedPercentage,
    ) -> anyhow::Result<()>;
    fn polarity(&mut self) -> anyhow::Result<Polarity>;
    fn set_polarity(&mut self, value: Polarity) -> anyhow::Result<()>;
    fn position(&mut self) -> anyhow::Result<TachoCounts>;
    fn set_position(&mut self, value: TachoCounts) -> anyhow::Result<()>;
    fn position_sp(&mut self) -> anyhow::Result<TachoCounts>;
    fn set_position_sp(&mut self, value: TachoCounts) -> anyhow::Result<()>;
    fn max_speed(&self) -> TachoCounts;
    fn state(&mut self) -> anyhow::Result<State>;
    fn speed(&mut self) -> anyhow::Result<TachoCounts>;
    fn speed_sp(&mut self) -> anyhow::Result<TachoCounts>;
    fn set_speed_sp(&mut self, value: TachoCounts) -> anyhow::Result<()>;
    fn stop_action(&mut self) -> anyhow::Result<StopAction>;
    fn set_stop_action(&mut self, value: StopAction) -> anyhow::Result<()>;
}
