use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Diff {
    pub operation: i32,
    pub text: String,
}

impl Diff {
    /// A new diff diff object created.
    pub fn new(operation: i32, text: String) -> Diff {
        Diff { operation, text }
    }
}

impl fmt::Debug for Diff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n  {{ {}: {} }}", self.operation, self.text)
    }
}