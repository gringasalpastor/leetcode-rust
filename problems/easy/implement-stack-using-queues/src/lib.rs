use std::collections::VecDeque;

pub struct MyStack {
    queue: VecDeque<i32>,
    top: i32,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack { queue: VecDeque::new(), top: -1 }
    }
    pub fn push(&mut self, x: i32) {
        self.queue.push_front(x);
        self.top = x;
    }
    pub fn pop(&mut self) -> i32 {
        for _ in 0..self.queue.len() - 1 {
            let x = self.queue.pop_back().expect("not empty");
            self.top = x;

            self.queue.push_front(x);
        }
        self.queue.pop_back().expect("not empty")
    }
    pub fn top(&self) -> i32 {
        self.top
    }
    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut stack = MyStack::new();

        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.empty(), false);
    }

    #[test]
    fn test2() {
        let mut stack = MyStack::new();

        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.top(), 1);
        assert_eq!(stack.empty(), false);
    }
}
