use std::vec::IntoIter;

pub struct ConsoleStack {
    stack: Vec<String>
}

impl ConsoleStack {
    pub fn new() -> ConsoleStack {
        ConsoleStack {
            stack: vec![]
        }
    }
}

impl ConsoleStack {
    pub fn clear(&mut self) {
        self.stack.clear()
    }

    pub fn push(&mut self, new_item: &str) {
        self.stack.push(new_item.to_string())
    }
}

impl<'a> ConsoleStack {
    pub fn get_recent_entries(&'a self, num: usize) -> impl Iterator<Item = &'a String> {
        self.stack.iter().take(num)
    }
}   