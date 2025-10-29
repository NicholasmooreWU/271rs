#[derive(Debug)]
pub struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Stack {
    pub vals: Option<Node>,
}

pub fn init() -> Stack {
    Stack { vals: None }
}

pub fn push(val: String, s: Stack) -> Stack {
    let next = s.vals.map(Box::new);

    let new_node = Node {
        data: val,
        next,
    };

    Stack {
        vals: Some(new_node),
    }
}

pub fn pop(s: Stack) -> (Option<String>, Stack) {
    match s.vals {
        None => (None, s),
        Some(node) => {
            let popped_value = Some(node.data);

            let new_head = match node.next {
                None => None,
                Some(boxed_node) => Some(*boxed_node),
            };

            (popped_value, Stack { vals: new_head })
        }
    }
}

