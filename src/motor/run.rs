pub trait Run<SpeedUnit: Clone> {
    fn run(&mut self, speed: SpeedUnit) -> anyhow::Result<()>;
}
