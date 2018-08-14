use std::time::Duration;

pub struct Report {
    pub concurrent: usize,
    pub total_time: Duration,
    pub ok_count: usize,
    pub err_count: usize
}

impl Report {
    pub fn total_count(&self) -> usize {
        self.ok_count + self.err_count
    }

    pub fn avg_time(&self) -> Duration {
        (self.total_time * self.concurrent as u32) / (self.total_count() as u32)
    }

    pub fn ok_rate(&self) -> f64 {
        (self.ok_count as f64) / (self.total_count() as f64)
    }

    pub fn error_rate(&self) -> f64 {
        (self.err_count as f64) / (self.total_count() as f64)
    }

    pub fn req_per_sec(&self) -> f64 {
        let secs = self.total_time.as_secs() as f64 + f64::from(self.total_time.subsec_nanos()) / 1_000_000_000.0;
        (self.total_count() as f64 ) / secs
    }
}
