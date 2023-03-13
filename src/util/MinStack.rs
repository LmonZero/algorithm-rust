/// 设计一个支持 push，pop，top 操作，并能在 <常数时间> 内检索到最小元素的栈。
/// MinStack() 初始化堆栈对象。
/// void push(int val) 将元素val推入堆栈。
/// void pop() 删除堆栈顶部的元素。
/// int top() 获取堆栈顶部的元素。
/// int getMin() 获取堆栈中的最小元素。

struct MinStack {
    min: Vec<i32>,
    stack: Vec<i32>, //临时 stack
}
impl MinStack {
    fn MinStack() -> MinStack {
        return MinStack {
            min: Vec::new(),
            stack: Vec::new(),
        };
    }

    fn push(&mut self, num: i32) {}

    fn pop(&mut self) {
        self.min.pop();
    }

    fn top(&self) -> i32 {
        return *self.min.last().unwrap();
    }

    fn getMin(&self) -> i32 {
        return *self.min.last().unwrap();
    }
}
