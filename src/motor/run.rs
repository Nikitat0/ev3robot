use crate::percentage::SignedPercentage;

pub trait Run<SpeedUnit: Clone> {
    fn run(&mut self, speed: SpeedUnit) -> anyhow::Result<()>;
}

pub trait RunDirect {
    fn run_direct(
        &mut self,
        duty_cycle: SignedPercentage,
    ) -> anyhow::Result<()>;
}
