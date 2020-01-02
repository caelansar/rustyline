use std::fmt;

pub trait Prompt: fmt::Debug {
    fn first_line(&self) -> &str;
    fn next_line(&self) -> &str { "" }
    fn bottom_line(&self) -> Option<&str> { None }
}

#[derive(Debug)]
pub struct SimplePrompt<'a>(pub &'a str);

impl<'a> Prompt for SimplePrompt<'a> {
    fn first_line(&self) -> &str {
        self.0
    }
}
