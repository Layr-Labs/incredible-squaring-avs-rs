//! cli crate  
use std::{future::Future, pin::pin};
use tracing::trace;

/// Incredible Runner
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct IncredibleRunner;

impl IncredibleRunner {
    /// Run until ctrl c
    pub fn run_blocking_until_ctrl_c<F, E>(self, fut: F) -> Result<(), E>
    where
        F: Future<Output = Result<(), E>> + Send + 'static,
        E: Send + Sync + From<std::io::Error> + 'static,
    {
        let tokio_runtime = tokio_runtime()?;
        let handle = tokio_runtime.handle().clone();
        let fut = tokio_runtime
            .handle()
            .spawn_blocking(move || handle.block_on(fut));
        tokio_runtime.block_on(run_until_ctrl_c(async move {
            fut.await.expect("Failed to join task")
        }))?;

        // drop the tokio runtime on a separate thread because drop blocks until its pools
        // (including blocking pool) are shutdown. In other words `drop(tokio_runtime)` would block
        // the current thread but we want to exit right away.
        std::thread::Builder::new()
            .name("tokio-runtime-shutdown".to_string())
            .spawn(move || drop(tokio_runtime))
            .unwrap();

        Ok(())
    }
}

/// Creates a new default tokio multi-thread [Runtime](tokio::runtime::Runtime) with all features
/// enabled
pub fn tokio_runtime() -> Result<tokio::runtime::Runtime, std::io::Error> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
}

/// Runs the future to completion or until:
/// - `ctrl-c` is received.
/// - `SIGTERM` is received (unix only).
async fn run_until_ctrl_c<F, E>(fut: F) -> Result<(), E>
where
    F: Future<Output = Result<(), E>>,
    E: Send + Sync + 'static + From<std::io::Error>,
{
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        let sigterm = stream.recv();
        let sigterm = pin!(sigterm);
        let ctrl_c = pin!(ctrl_c);
        let fut = pin!(fut);

        tokio::select! {
            _ = ctrl_c => {
                trace!(target: "incredible::cli", "Received ctrl-c");
            },
            _ = sigterm => {
                trace!(target: "incredible::cli", "Received SIGTERM");
            },
            res = fut => res?,
        }
    }

    #[cfg(not(unix))]
    {
        let ctrl_c = pin!(ctrl_c);
        let fut = pin!(fut);

        tokio::select! {
            _ = ctrl_c => {
                trace!(target: "incredible::cli", "Received ctrl-c");
            },
            res = fut => res?,
        }
    }

    Ok(())
}
