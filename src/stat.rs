use hyper;

#[derive(Debug)]
pub struct Stat {
    pub total: usize,
    pub ok_num: usize,
    pub err_num: usize,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            total: 0,
            ok_num: 0,
            err_num: 0
        }
    }

    pub fn add(&mut self, response: hyper::Response) {
        self.total += 1;

        if response.status().is_success() {
            self.ok_num += 1;
        } else {
            self.err_num += 1;
        }
    }

    pub fn ok_rate(&self) -> f64 {
        (self.ok_num as f64) / (self.total as f64)
    }

    pub fn error_rate(&self) -> f64 {
        (self.err_num as f64) / (self.total as f64)
    }
}
