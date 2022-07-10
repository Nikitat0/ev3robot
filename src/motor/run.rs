pub trait Run<Speed> {
    fn run(speed: Speed) -> anyhow::Result<()>;
}
