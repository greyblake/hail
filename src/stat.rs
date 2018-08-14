use hyper;

#[derive(Debug)]
pub struct Stat {
    pub ok_num: usize,
    pub err_num: usize,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            ok_num: 0,
            err_num: 0
        }
    }

    pub fn add(&mut self, response: hyper::Response) {
        if response.status().is_success() {
            self.ok_num += 1;
        } else {
            self.err_num += 1;
        }
    }
}
