use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

pub struct ThreadPool {
    pool: Runtime,
}

impl ThreadPool {
    pub fn new(threads_override: Option<usize>, pool_override: Option<Runtime>) -> ThreadPool {
        let pool = pool_override
            .or(Some(ThreadPool::create_pool(threads_override)))
            .unwrap();
        ThreadPool { pool }
    }

    pub fn spawn<T: Send + 'static, F: (Fn() -> T) + 'static + Send>(
        &mut self,
        task: F,
    ) -> JoinHandle<T> {
        return self.pool.block_on(async {
            return tokio::spawn(async move { task() });
        });
    }

    fn create_pool(threads: Option<usize>) -> Runtime {
        let mut pool = runtime::Builder::new_multi_thread();
        pool.enable_all();
        match threads {
            Some(size) => &pool.worker_threads(size),
            None => &pool,
        };
        return pool.build().unwrap();
    }
}
