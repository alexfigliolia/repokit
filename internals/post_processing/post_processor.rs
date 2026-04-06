use std::sync::{LazyLock, Mutex, MutexGuard};

pub struct PostProcessor {
    tasks: Vec<Box<dyn Fn() + Send + 'static>>,
}

impl PostProcessor {
    pub fn new() -> Self {
        PostProcessor { tasks: Vec::new() }
    }

    pub fn get() -> MutexGuard<'static, PostProcessor> {
        REPOKIT_POST_PROCESSOR.lock().unwrap()
    }

    pub fn register_task<F>(&mut self, task: F)
    where
        F: Fn() + Send + 'static,
    {
        self.tasks.push(Box::new(task));
    }

    pub fn flush(&mut self) {
        for task in self.tasks.iter() {
            task();
        }
        self.tasks.clear();
        self.tasks.shrink_to_fit();
    }
}

static REPOKIT_POST_PROCESSOR: LazyLock<Mutex<PostProcessor>> =
    LazyLock::new(|| Mutex::new(PostProcessor::new()));
