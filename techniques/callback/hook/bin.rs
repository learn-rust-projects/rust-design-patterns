use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Mutex;

type AsyncHook = Box<dyn Fn() -> tokio::task::JoinHandle<()> + Send + Sync>;

struct AsyncShutdownHooks {
    hooks: Mutex<Vec<AsyncHook>>,
}

impl AsyncShutdownHooks {
    fn new() -> Self {
        Self {
            hooks: Mutex::new(Vec::new()),
        }
    }

    fn add_hook<Fut, F>(&self, hook: F)
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let wrapped_hook = Box::new(move || tokio::spawn(hook())) as AsyncHook;

        let mut hooks = futures::executor::block_on(self.hooks.lock());
        hooks.push(wrapped_hook);
    }

    async fn run_hooks(&self) {
        let hooks = {
            let mut hooks_lock = self.hooks.lock().await;
            std::mem::take(&mut *hooks_lock)
        };

        println!("Running {} async shutdown hooks...", hooks.len());

        let tasks: Vec<_> = hooks.into_iter().map(|hook| hook()).collect();
        join_all(tasks).await;
    }
}

#[tokio::main]
async fn main() {
    let shutdown_hooks = Arc::new(AsyncShutdownHooks::new());

    shutdown_hooks.add_hook(|| async {
        println!("Closing async database connections...");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        println!("Async database connections closed.");
    });

    shutdown_hooks.add_hook(|| async {
        println!("Flushing async logs...");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Async logs flushed.");
    });

    let hooks_for_signal = shutdown_hooks.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl-C");
        println!("Received termination signal, starting async shutdown...");
        hooks_for_signal.run_hooks().await;
        println!("Async shutdown hooks finished. Exiting now.");
        std::process::exit(0);
    });

    println!("Async application running. Press Ctrl-C to exit.");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("Async service running...");
    }
}
