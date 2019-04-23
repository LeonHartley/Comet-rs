pub trait SearchHandler<T> {
    fn search(&self, view: T);
}
