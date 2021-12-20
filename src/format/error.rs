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

    pub fn get_error(&self) -> i64 {
        return self.err;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}