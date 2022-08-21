pub trait Run<Speed> {
    fn run(&mut self, speed: Speed) -> anyhow::Result<()>;
}
