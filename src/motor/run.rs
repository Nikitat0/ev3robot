use crate::percentage::SignedPercentage;

pub trait Run<SpeedUnit: Clone> {
    fn run(&mut self, speed: SpeedUnit) -> anyhow::Result<()>;
}

pub trait RunDirect {
    fn run_direct(
        &mut self,
        duty_cycle: SignedPercentage,
    ) -> anyhow::Result<DutyCycleController>;
}

pub struct DutyCycleController<'a>(
    Box<dyn FnMut(SignedPercentage) -> anyhow::Result<()> + 'a>,
);

impl<'a> DutyCycleController<'a> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(SignedPercentage) -> anyhow::Result<()> + 'a,
    {
        Self(Box::new(f))
    }

    pub fn set_duty_cycle(
        &mut self,
        value: SignedPercentage,
    ) -> anyhow::Result<()> {
        self.0(value)
    }
}
