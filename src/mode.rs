pub trait Mode: Sized {
    type Of;

    fn of(_: &mut Self::Of) -> Self;
}
