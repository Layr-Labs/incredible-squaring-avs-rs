//! Metrics for the incredible squaring task manager
use eigen_metrics_derive::Metrics;
use metrics::{Counter, IntoF64};
use std::sync::atomic::{AtomicU64, Ordering};

/// Counter metrics
#[derive(Metrics)]
#[metrics(scope = "eigenmetrics.incredible_squaring_metrics")]
pub struct CounterMetrics {
    /// number of tasks received
    #[metric(
        rename = "num_tasks_received",
        describe = "The number of tasks received by reading from the avs service manager contract"
    )]
    pub num_tasks_received: Counter,
}

/// Counter Metrics that are used to track the number of tasks received
#[derive(Debug)]
pub struct IncredibleMetrics {
    /// [`CounterMetrics`]
    pub counter: CounterMetrics,

    internal_count: AtomicU64,
}

impl IncredibleMetrics {
    /// Create a new instance of the metrics
    pub fn new() -> Self {
        let counter = Self {
            counter: CounterMetrics {
                num_tasks_received: metrics::register_counter!("num_tasks_received"),
            },
            internal_count: AtomicU64::new(0),
        };
        CounterMetrics::describe();
        counter
    }

    /// Get the number of tasks received
    pub fn num_tasks_received(&self) -> u64 {
        self.internal_count.load(Ordering::Relaxed)
    }

    /// Increment the number of tasks received
    pub fn increment_num_tasks_received(&self) {
        self.counter
            .num_tasks_received
            .increment(self.num_tasks_received());
        self.internal_count.fetch_add(1, Ordering::Relaxed);
    }
}
