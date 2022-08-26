pub trait Run<SpeedUnit> {
    fn run(&mut self, speed: SpeedUnit) -> anyhow::Result<()>;
}
