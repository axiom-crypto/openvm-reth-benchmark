use std::time::Instant;

macro_rules! time {
    ($name:expr, $block:expr) => {{
        let _timer = crate::utils::Timer::new($name);
        $block
    }};
}
pub(crate) use time;

pub(crate) struct Timer {
    name: &'static str,
    start: Instant,
}

impl Timer {
    pub(crate) fn new(name: &'static str) -> Self {
        tracing::trace!("{} started", name);
        Self { name, start: Instant::now() }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        tracing::trace!(?elapsed, "{} finished", self.name);

        let metric_name = format!("reth_witness.{}_duration_ms", self.name);
        metrics::histogram!(metric_name).record(elapsed.as_millis() as f64);
    }
}
