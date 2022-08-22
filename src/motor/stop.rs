pub trait Coast {
    fn coast(&mut self) -> anyhow::Result<()>;
}

pub trait Brake {
    fn brake(&mut self) -> anyhow::Result<()>;
}

pub trait Hold {
    fn hold(&mut self) -> anyhow::Result<()>;
}
