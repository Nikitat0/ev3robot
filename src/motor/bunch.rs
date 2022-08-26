use derive_more::*;

use super::{Brake, Coast, Hold, IsHolding, IsRunning, Run};

#[derive(Debug, Index, IndexMut, IntoIterator)]
pub struct MotorsBunch<Motor>(Vec<Motor>);

impl<Motor> MotorsBunch<Motor> {
    pub fn new<I: IntoIterator<Item = Motor>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    pub fn exec<T, U, F>(&mut self, f: F) -> anyhow::Result<U>
    where
        F: FnMut(&mut Motor) -> anyhow::Result<T>,
        U: FromIterator<T>,
    {
        self.0.iter_mut().map(f).collect()
    }
}

impl<Motor> FromIterator<Motor> for MotorsBunch<Motor> {
    fn from_iter<I: IntoIterator<Item = Motor>>(iter: I) -> Self {
        MotorsBunch(iter.into_iter().collect())
    }
}

impl<Motor, SpeedUnit> Run<SpeedUnit> for MotorsBunch<Motor>
where
    Motor: Run<SpeedUnit>,
    SpeedUnit: Clone,
{
    fn run(&mut self, speed: SpeedUnit) -> anyhow::Result<()> {
        self.exec(|it| it.run(speed.clone()))
    }
}

impl<Motor: IsRunning> IsRunning for MotorsBunch<Motor> {
    fn is_running(&mut self) -> anyhow::Result<bool> {
        let mut is_running = false;
        self.exec(|it| it.is_running().map(|b| is_running |= b))?;
        Ok(is_running)
    }
}

impl<Motor: IsHolding> IsHolding for MotorsBunch<Motor> {
    fn is_holding(&mut self) -> anyhow::Result<bool> {
        let mut is_holding = false;
        self.exec(|it| it.is_holding().map(|b| is_holding |= b))?;
        Ok(is_holding)
    }
}

impl<Motor: Coast> Coast for MotorsBunch<Motor> {
    fn coast(&mut self) -> anyhow::Result<()> {
        self.exec(Coast::coast)
    }
}

impl<Motor: Brake> Brake for MotorsBunch<Motor> {
    fn brake(&mut self) -> anyhow::Result<()> {
        self.exec(Brake::brake)
    }
}

impl<Motor: Hold> Hold for MotorsBunch<Motor> {
    fn hold(&mut self) -> anyhow::Result<()> {
        self.exec(Hold::hold)
    }
}
