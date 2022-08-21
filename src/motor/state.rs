pub trait IsRunning {
    fn is_running(&mut self) -> anyhow::Result<bool>;
}

pub trait IsHolding {
    fn is_holding(&mut self) -> anyhow::Result<bool>;
}
