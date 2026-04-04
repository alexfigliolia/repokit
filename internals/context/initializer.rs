use std::future::Future;

use futures::executor::block_on;

pub trait Initializer<T, P> {
    async fn resolve(&mut self, input: P) -> T;

    fn resolve_sync<R>(future: impl Future<Output = R>) -> R {
        block_on(future)
    }
}
