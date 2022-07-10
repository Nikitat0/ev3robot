use derive_more::Display;

pub trait Stop<T> {
    fn stop(_: T);
}
