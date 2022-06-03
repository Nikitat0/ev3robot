mod command;
mod polarity;
mod state;
mod stop_action;

pub use command::*;
pub use polarity::*;
pub use state::*;
pub use stop_action::*;

use crate as ev3robot;
use crate::device::{
    ReadOnlyAttributeFile, ReadWriteAttributeFile, WriteOnlyAttributeFile,
};

#[derive(Debug, Device)]
#[ev3robot(class = "tacho-motor")]
pub struct TachoMotorDevice {
    command: WriteOnlyAttributeFile<Command>,
    count_per_rot: u32,
    duty_cycle: ReadOnlyAttributeFile<i8>,
    duty_cycle_sp: ReadWriteAttributeFile<i8>,
    polarity: ReadWriteAttributeFile<Polarity>,
    position: ReadWriteAttributeFile<i32>,
    position_sp: ReadWriteAttributeFile<i32>,
    max_speed: u32,
    state: ReadOnlyAttributeFile<State>,
    speed: ReadOnlyAttributeFile<i32>,
    speed_sp: ReadWriteAttributeFile<i32>,
    stop_action: ReadWriteAttributeFile<StopAction>,
}
