/// 设计一个支持 push，pop，top 操作，并能在 <常数时间> 内检索到最小元素的栈。
/// MinStack() 初始化堆栈对象。
/// void push(int val) 将元素val推入堆栈。
/// void pop() 删除堆栈顶部的元素。
/// int top() 获取堆栈顶部的元素。
/// int getMin() 获取堆栈中的最小元素。

pub struct MinStack {
    min_stack: Vec<i32>, //保持栈顶最小
    stack: Vec<i32>,     //正常进栈出栈
}
impl MinStack {
    fn new() -> Self {
        MinStack {
            min_stack: Vec::new(),
            stack: Vec::new(),
        }
    }

    fn push(&mut self, num: i32) {
        if self.get_min() > num {
            self.min_stack.push(num)
        }

        self.stack.push(num);
    }

    fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        //遇到辅助栈顶 相同  一起出栈
        if self.stack.pop().unwrap() == self.get_min() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        if self.stack.is_empty() {
            return i32::MAX;
        }

        return *self.stack.last().unwrap();
    }

    fn get_min(&self) -> i32 {
        if self.min_stack.is_empty() {
            return i32::MAX;
        }
        return *self.min_stack.last().unwrap();
    }
}

mod tests {
    use super::MinStack;

    #[test]
    fn min_stack_test() {
        let mut stack = MinStack::new();
        println!("{}", stack.get_min());
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        println!("1 {}", stack.get_min());
        println!("1 {}", stack.top());

        stack.pop();
        println!("2 {}", stack.get_min());
        println!("2 {}", stack.top());

        stack.pop();
        println!("3 {}", stack.get_min());
        println!("3 {}", stack.top());

        stack.pop();
        println!("4 {}", stack.get_min());
        println!("4 {}", stack.top());
    }
}
