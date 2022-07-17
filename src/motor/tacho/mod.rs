mod command;
mod state;
mod units;

pub use command::*;
pub use state::*;
pub use units::*;

use super::*;
use crate::device::{
    ReadOnlyAttributeFile, ReadWriteAttributeFile, WriteOnlyAttributeFile,
};

#[allow(unused)]
#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "tacho-motor")]
pub struct TachoMotor {
    command: WriteOnlyAttributeFile,
    count_per_rot: i32,
    duty_cycle: ReadOnlyAttributeFile,
    duty_cycle_sp: ReadWriteAttributeFile,
    polarity: ReadWriteAttributeFile,
    position: ReadWriteAttributeFile,
    position_sp: ReadWriteAttributeFile,
    max_speed: i32,
    state: ReadOnlyAttributeFile,
    speed: ReadOnlyAttributeFile,
    speed_sp: ReadWriteAttributeFile,
    stop_action: ReadWriteAttributeFile,
}

impl TachoMotor {
    fn run(&mut self, speed: PerSecond<TachoCounts>) -> anyhow::Result<()> {
        self.speed_sp.set_value(speed.units().as_i32())?;
        self.command.set_value("run-forever")?;
        Ok(())
    }

    fn stop(&mut self, stop_action: String) -> anyhow::Result<()> {
        self.stop_action.set_value(stop_action)?;
        self.command.set_value("stop")?;
        Ok(())
    }

    fn degrees_to_tacho_counts(&self, degrees: Degrees) -> TachoCounts {
        TachoCounts::new((self.count_per_rot * degrees.as_i32() / 360) as i32)
    }

    fn revolutions_to_tacho_counts(
        &self,
        revolutions: Revolutions,
    ) -> TachoCounts {
        TachoCounts::new(
            (self.count_per_rot as f32 * revolutions.as_f32()) as i32,
        )
    }
}

impl Run<PerSecond<TachoCounts>> for TachoMotor {
    fn run(&mut self, speed: PerSecond<TachoCounts>) -> anyhow::Result<()> {
        self.run(speed)
    }
}

impl Run<PerSecond<Degrees>> for TachoMotor {
    fn run(&mut self, speed: PerSecond<Degrees>) -> anyhow::Result<()> {
        self.run(PerSecond::new(self.degrees_to_tacho_counts(speed.units())))
    }
}

impl Run<PerSecond<Revolutions>> for TachoMotor {
    fn run(&mut self, speed: PerSecond<Revolutions>) -> anyhow::Result<()> {
        self.run(PerSecond::new(
            self.revolutions_to_tacho_counts(speed.units()),
        ))
    }
}

macro_rules! tacho_motor_stop_action {
    ($stop_action:ty) => {
        impl Stop<$stop_action> for TachoMotor {
            fn stop(
                &mut self,
                stop_action: $stop_action,
            ) -> anyhow::Result<()> {
                self.stop(stop_action.to_string())
            }
        }
    };
}

tacho_motor_stop_action!(Brake);
tacho_motor_stop_action!(Coast);
tacho_motor_stop_action!(Hold);
