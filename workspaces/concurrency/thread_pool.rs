use tokio::{
    runtime::{self, Runtime},
    task::JoinHandle,
};

pub struct ThreadPool {
    pub pool: Runtime,
}

impl ThreadPool {
    pub fn new(threads_override: Option<usize>, pool_override: Option<Runtime>) -> ThreadPool {
        let pool = pool_override.unwrap_or(ThreadPool::create_pool(threads_override));
        ThreadPool { pool }
    }

    pub fn spawn<T: Send + 'static, F: (Fn() -> T) + 'static + Send>(
        &mut self,
        task: F,
    ) -> JoinHandle<T> {
        self.pool.spawn(async move { task() })
    }

    fn create_pool(threads: Option<usize>) -> Runtime {
        let mut pool = runtime::Builder::new_multi_thread();
        pool.enable_all();
        match threads {
            Some(size) => pool.worker_threads(size),
            None => &pool,
        };
        pool.build().unwrap()
    }
}
