#[derive(Debug, Clone)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub trait Push<T> {
    fn push(self, val: T) -> Self;
}

pub trait Pop<T> {
    fn pop(self) -> (Option<T>, Self);
}

impl<T> Push<T> for Stack<T> {
    fn push(mut self, val: T) -> Self {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self
    }
}

impl<T> Pop<T> for Stack<T> {
    fn pop(mut self) -> (Option<T>, Self) {
        match self.head.take() {
            Some(boxed_node) => {
                let Node { val, next } = *boxed_node;
                (Some(val), Stack { head: next })
            }
            None => (None, self),
        }
    }
}

pub fn stack<T>() -> Stack<T> {
    Stack::new()
}

#[derive(Debug, Clone)]
pub struct Queue<T> {
    front: Stack<T>,
    back: Stack<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            front: Stack::new(),
            back: Stack::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

impl<T> Push<T> for Queue<T> {
    fn push(self, val: T) -> Self {
        Queue {
            front: self.front,
            back: self.back.push(val),
        }
    }
}

impl<T> Pop<T> for Queue<T> {
    fn pop(self) -> (Option<T>, Self) {
        let (maybe_val, front_after) = self.front.pop();
        if maybe_val.is_some() {
            return (
                maybe_val,
                Queue {
                    front: front_after,
                    back: self.back,
                },
            );
        }

        let mut b = self.back;
        let mut f = front_after;
        loop {
            let (opt_v, b_after) = b.pop();
            b = b_after;
            if let Some(v) = opt_v {
                f = f.push(v);
            } else {
                break;
            }
        }

        let (maybe2, f_after) = f.pop();
        (
            maybe2,
            Queue {
                front: f_after,
                back: b,
            },
        )
    }
}

pub fn queue<T>() -> Queue<T> {
    Queue::new()
}

