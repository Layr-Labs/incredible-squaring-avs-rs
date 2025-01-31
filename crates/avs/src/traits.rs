use std::future::Future;

/// Launch Avs trait
pub trait LaunchAvs<T: Send + 'static> {
    /// Launch Avs
    fn launch_avs(avs: T) -> impl Future<Output = eyre::Result<()>> + Send;
}
