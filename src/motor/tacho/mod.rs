mod command;
mod state;
mod stop_action;
mod units;

pub use command::*;
pub use state::*;
pub use stop_action::*;
pub use units::*;

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
    count_per_rot: TachoCounts,
    duty_cycle: ReadOnlyAttributeFile,
    duty_cycle_sp: ReadWriteAttributeFile,
    polarity: ReadWriteAttributeFile,
    position: ReadWriteAttributeFile,
    position_sp: ReadWriteAttributeFile,
    max_speed: TachoCounts,
    state: ReadOnlyAttributeFile,
    speed: ReadOnlyAttributeFile,
    speed_sp: ReadWriteAttributeFile,
    stop_action: ReadWriteAttributeFile,
}

pub trait TachoMotorInterface {
    fn command(&mut self, value: Command) -> anyhow::Result<()>;
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

impl TachoMotorInterface for TachoMotor {
    fn command(&mut self, value: Command) -> anyhow::Result<()> {
        self.command.set_value(value).map_err(anyhow::Error::new)
    }

    fn count_per_rot(&self) -> TachoCounts {
        self.count_per_rot
    }

    fn duty_cycle(&mut self) -> anyhow::Result<SignedPercentage> {
        self.duty_cycle.value().map_err(anyhow::Error::new)
    }

    fn duty_cycle_sp(&mut self) -> anyhow::Result<SignedPercentage> {
        self.duty_cycle_sp.value().map_err(anyhow::Error::new)
    }

    fn set_duty_cycle_sp(
        &mut self,
        value: SignedPercentage,
    ) -> anyhow::Result<()> {
        self.duty_cycle_sp.set_value(value).map_err(anyhow::Error::new)
    }

    fn polarity(&mut self) -> anyhow::Result<Polarity> {
        self.polarity.value().map_err(anyhow::Error::new)
    }

    fn set_polarity(&mut self, value: Polarity) -> anyhow::Result<()> {
        self.polarity.set_value(value).map_err(anyhow::Error::new)
    }

    fn position(&mut self) -> anyhow::Result<TachoCounts> {
        self.position.value().map_err(anyhow::Error::new)
    }

    fn set_position(&mut self, value: TachoCounts) -> anyhow::Result<()> {
        self.position.set_value(value).map_err(anyhow::Error::new)
    }

    fn position_sp(&mut self) -> anyhow::Result<TachoCounts> {
        self.position_sp.value().map_err(anyhow::Error::new)
    }

    fn set_position_sp(&mut self, value: TachoCounts) -> anyhow::Result<()> {
        self.position_sp.set_value(value).map_err(anyhow::Error::new)
    }

    fn max_speed(&self) -> TachoCounts {
        self.max_speed
    }

    fn state(&mut self) -> anyhow::Result<State> {
        self.state.value().map_err(anyhow::Error::new)
    }

    fn speed(&mut self) -> anyhow::Result<TachoCounts> {
        self.speed.value().map_err(anyhow::Error::new)
    }

    fn speed_sp(&mut self) -> anyhow::Result<TachoCounts> {
        self.speed_sp.value().map_err(anyhow::Error::new)
    }

    fn set_speed_sp(&mut self, value: TachoCounts) -> anyhow::Result<()> {
        self.speed_sp.set_value(value).map_err(anyhow::Error::new)
    }

    fn stop_action(&mut self) -> anyhow::Result<StopAction> {
        self.stop_action.value().map_err(anyhow::Error::new)
    }

    fn set_stop_action(&mut self, value: StopAction) -> anyhow::Result<()> {
        self.stop_action.set_value(value).map_err(anyhow::Error::new)
    }
}
