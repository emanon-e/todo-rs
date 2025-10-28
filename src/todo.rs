use colored::Colorize;
use core::fmt;

pub struct Todo {
    pub id: u32,
    pub text: String,
    pub is_completed: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_completed {
            true => write!(f, "{}", self.text.strikethrough().dimmed()),
            false => write!(f, "{}", &self.text),
        }
    }
}
