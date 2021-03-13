use std::cmp;

pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push((x, cmp::min(self.get_min(), x)));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().expect("Stack not empty").0
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap_or(&(i32::max_value(), i32::max_value())).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_stack() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}
