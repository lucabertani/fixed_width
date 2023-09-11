#[derive(Debug)]
pub struct FixedWidthError {
    msg: String,
}

impl FixedWidthError {
    pub fn new<S: Into<String>>(msg: S) -> FixedWidthError {
        FixedWidthError { msg: msg.into() }
    }

    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }
}
