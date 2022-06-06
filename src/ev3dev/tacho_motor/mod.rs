mod command;
mod polarity;
mod state;
mod stop_action;

use std::io;

pub use command::*;
pub use polarity::*;
pub use state::*;
pub use stop_action::*;

use crate as ev3robot;
use crate::device::{
    ReadOnlyAttributeFile, ReadWriteAttributeFile, WriteOnlyAttributeFile,
};

#[derive(Debug, Device)]
pub struct TachoMotor {
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

impl TachoMotor {
    pub fn command(&self, command: Command) -> io::Result<()> {
        self.command.set_value(command)
    }

    pub fn count_per_rot(&self) -> u32 {
        self.count_per_rot
    }

    pub fn duty_cycle(&self) -> io::Result<i8> {
        self.duty_cycle.value()
    }

    pub fn duty_cycle_sp(&self) -> io::Result<i8> {
        self.duty_cycle_sp.value()
    }

    pub fn set_duty_cycle_sp(&self, duty_cycle: i8) -> io::Result<()> {
        self.duty_cycle_sp.set_value(duty_cycle)
    }

    pub fn polarity(&self) -> io::Result<Polarity> {
        self.polarity.value()
    }

    pub fn set_polarity(&self, polarity: Polarity) -> io::Result<()> {
        self.polarity.set_value(polarity)
    }

    pub fn position(&self) -> io::Result<i32> {
        self.position.value()
    }

    pub fn set_position(&self, pos: i32) -> io::Result<()> {
        self.position.set_value(pos)
    }

    pub fn position_sp(&self) -> io::Result<i32> {
        self.position_sp.value()
    }

    pub fn set_position_sp(&self, pos: i32) -> io::Result<()> {
        self.position_sp.set_value(pos)
    }

    pub fn state(&self) -> io::Result<State> {
        self.state.value()
    }

    pub fn max_speed(&self) -> u32 {
        self.max_speed
    }

    pub fn speed(&self) -> io::Result<i32> {
        self.speed.value()
    }

    pub fn speed_sp(&self) -> io::Result<i32> {
        self.speed_sp.value()
    }

    pub fn set_speed_sp(&self, speed: i32) -> io::Result<()> {
        self.speed_sp.set_value(speed)
    }

    pub fn stop_action(&self) -> io::Result<StopAction> {
        self.stop_action.value()
    }

    pub fn set_stop_action(&self, stop_action: StopAction) -> io::Result<()> {
        self.stop_action.set_value(stop_action)
    }
}

pub trait AsTachoMotor {
    fn as_tacho_motor(&self) -> &TachoMotor;
}
