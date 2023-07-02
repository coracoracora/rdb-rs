use formatter::Formatter;

#[derive(Default)]
pub struct Nil;

impl Nil {
    pub fn new() -> Nil {
        Nil
    }
}

impl Formatter for Nil {}
