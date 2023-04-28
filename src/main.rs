use vector_array::vec::VecArray;
use vector_array::vec_arr;

#[allow(warnings)]
#[derive(Debug, Clone)]
struct StackTooSmall;

#[derive(Debug, Clone, PartialEq)]
struct Stack<T, const CAP: usize> {
    stack: VecArray<T, CAP>,
}

impl<T, const CAP: usize> Stack<T, CAP>
where
    T: Default,
{
    fn new() -> Self {
        Stack {
            stack: VecArray::new(),
        }
    }
}

impl<T, const CAP: usize> Stack<T, CAP> {
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) -> Result<(), StackTooSmall> {
        self.stack.push(item).map_err(|_| StackTooSmall)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main() {
    let mut vec: VecArray<String, 4> = vec_arr![
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string(),
        "Hello".to_string()
    ];
    vec.push("Hello".to_owned()).unwrap();
    // dbg!(&vec[0]);
}
