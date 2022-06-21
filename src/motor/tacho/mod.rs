mod command;
mod state;
mod stop_action;
use std::io;

pub use command::*;
pub use state::*;
pub use stop_action::*;

use super::duty_cycle::*;
use super::polarity::*;
use crate::device::{
    ReadOnlyAttributeFile, ReadWriteAttributeFile, WriteOnlyAttributeFile,
};

#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "tacho-motor")]
#[device(apply = "TachoMotor::reset_motor")]
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
    fn reset_motor(&mut self) -> io::Result<()> {
        self.command(Command::Reset)
    }
}

impl TachoMotor {
    pub fn command(&mut self, command: Command) -> io::Result<()> {
        self.command.set_value(command)
    }

    pub fn count_per_rot(&mut self) -> u32 {
        self.count_per_rot
    }

    pub fn duty_cycle(&mut self) -> io::Result<DutyCycle> {
        self.duty_cycle.value()
    }

    pub fn duty_cycle_sp(&mut self) -> io::Result<DutyCycle> {
        self.duty_cycle_sp.value()
    }

    pub fn set_duty_cycle_sp(
        &mut self,
        duty_cycle: DutyCycle,
    ) -> io::Result<()> {
        self.duty_cycle_sp.set_value(duty_cycle)
    }

    pub fn polarity(&mut self) -> io::Result<Polarity> {
        self.polarity.value()
    }

    pub fn set_polarity(&mut self, polarity: Polarity) -> io::Result<()> {
        self.polarity.set_value(polarity)
    }

    pub fn position(&mut self) -> io::Result<i32> {
        self.position.value()
    }

    pub fn set_position(&mut self, pos: i32) -> io::Result<()> {
        self.position.set_value(pos)
    }

    pub fn position_sp(&mut self) -> io::Result<i32> {
        self.position_sp.value()
    }

    pub fn set_position_sp(&mut self, pos: i32) -> io::Result<()> {
        self.position_sp.set_value(pos)
    }

    pub fn state(&mut self) -> io::Result<State> {
        self.state.value()
    }

    pub fn max_speed(&mut self) -> u32 {
        self.max_speed
    }

    pub fn speed(&mut self) -> io::Result<i32> {
        self.speed.value()
    }

    pub fn speed_sp(&mut self) -> io::Result<i32> {
        self.speed_sp.value()
    }

    pub fn set_speed_sp(&mut self, speed: i32) -> io::Result<()> {
        self.speed_sp.set_value(speed)
    }

    pub fn stop_action(&mut self) -> io::Result<StopAction> {
        self.stop_action.value()
    }

    pub fn set_stop_action(
        &mut self,
        stop_action: StopAction,
    ) -> io::Result<()> {
        self.stop_action.set_value(stop_action)
    }
}
