use super::stop::Stop;

pub trait Run<Speed> {
    fn run(&mut self, speed: Speed) -> anyhow::Result<()>;
}

pub trait RunFor<T, Speed, StopAction>: Run<Speed> + Stop<StopAction> {
    fn run_for(
        &mut self,
        _: T,
        speed: Speed,
        stop_action: StopAction,
    ) -> anyhow::Result<()>;
}
