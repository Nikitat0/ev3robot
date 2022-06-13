pub trait Mode: Sized {
    type Of;

    fn of(_: &mut Self::Of) -> Self;
}

pub trait ModeExt {
    fn mode<M>(&mut self) -> M
    where
        M: Mode<Of = Self>,
    {
        Mode::of(self)
    }
}

impl<T> ModeExt for T {}
