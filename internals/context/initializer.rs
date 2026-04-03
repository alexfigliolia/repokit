use std::future::Future;

use futures::executor::block_on;

pub trait Initializer<T> {
    async fn resolve(&mut self, known_root: &str) -> T;

    fn resolve_sync<R>(future: impl Future<Output = R>) -> R {
        block_on(future)
    }
}
