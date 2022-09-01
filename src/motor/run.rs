use crate::percentage::SignedPercentage;

pub trait Run<SpeedUnit: Clone> {
    fn run(&mut self, speed: SpeedUnit) -> anyhow::Result<()>;
}

pub trait RunDirect {
    fn run_direct<'a>(
        &'a mut self,
        duty_cycle: SignedPercentage,
    ) -> anyhow::Result<Box<dyn DutyCycleController + 'a>>;
}

pub trait DutyCycleController {
    fn set_duty_cycle(&mut self, value: SignedPercentage)
        -> anyhow::Result<()>;
}

impl<F> DutyCycleController for F
where
    F: FnMut(SignedPercentage) -> anyhow::Result<()>,
{
    fn set_duty_cycle(
        &mut self,
        value: SignedPercentage,
    ) -> anyhow::Result<()> {
        self(value)
    }
}
