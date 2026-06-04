pub trait AsyncScope<T> {
    async fn new() -> Self;
    async fn resolve() -> T;
}
