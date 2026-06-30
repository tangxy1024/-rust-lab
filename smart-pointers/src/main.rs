fn main() {
    println!("=== Rust 智能指针 ===");
    println!("在这里练习：Box、Rc、Arc、RefCell、Mutex、Deref trait");

    let a = Rc::new(Node { next: None });
    let b = Rc::new(Node { next: Some(a.clone()) });
}

use std::rc::Rc;

struct Node {
    next: Option<Rc<Node>>,
}
