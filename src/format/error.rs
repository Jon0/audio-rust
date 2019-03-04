#[derive(Debug)]
pub struct DriverError {
    err: i64,
    name: String,
    desc: String,
}

impl DriverError {
    pub fn new(e: i64, n: &str, d: &str) -> DriverError {
        DriverError {  err: e, name: String::from(n), desc: String::from(d) }
    }

    pub fn as_string(&self) -> String {
        return self.desc.clone();
    }
}