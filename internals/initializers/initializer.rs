use std::future::Future;

use futures::executor::block_on;

pub trait Initializer<T> {
    async fn resolve(&mut self, known_root: &str);

    fn resolve_sync(future: impl Future<Output = T>) {
        block_on(future);
    }
}
