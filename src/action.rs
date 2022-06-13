pub trait Action<T> {
    type Result;

    fn execute(&self, on: &mut T) -> Self::Result;
}
