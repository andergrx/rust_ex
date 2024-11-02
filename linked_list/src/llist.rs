use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new() {}
}

type NodePointer<T> = Rc<RefCell<NodeD<T>>>;

#[derive(Debug)]
pub struct NodeD<T> {
    pub data: T,
    pub next: Option<NodePointer<T>>,
    pub prev: Option<NodePointer<T>>,
}

impl<T> NodeD<T> {

    pub fn new_ptr(data: T) -> NodePointer<T> {
        Rc::new(RefCell::new(
            NodeD {
                data: data,
                next: None,
                prev: None,
            }
        ))
    }

    pub fn new_ptr_with_next(data: T, next: NodePointer<T>) -> NodePointer<T> {
        Rc::new(RefCell::new(
            NodeD {
                data: data,
                next: Some(next),
                prev: None,
            }
        ))
    }

    pub fn new_ptr_with_prev(data: T, prev: NodePointer<T>) -> NodePointer<T> {
        Rc::new(RefCell::new(
            NodeD {
                data: data,
                next: None,
                prev: Some(prev),
            }
        ))
    }

    pub fn next(&self) -> NodePointer<T> {
        Rc::clone(self.next.as_ref().unwrap())
    }

    pub fn get_node(n_ptr: &NodePointer<T>) -> RefMut<NodeD<T>> {
        n_ptr.borrow_mut()
    }

    pub fn get_next(n_ptr: &NodePointer<T>) -> NodePointer<T> {
        n_ptr.borrow().next.as_ref().unwrap().clone()
    }

    pub fn get_prev(n_ptr: &NodePointer<T>) -> NodePointer<T> {
        n_ptr.borrow().prev.as_ref().unwrap().clone()
    }
}

// #[derive(Debug)]
// pub struct NodeD<'a, T> {
//     pub data: T,
//     pub next: Option<Rc<&'a NodeD<'a, T>>>,
//     pub prev: Option<Rc<&'a NodeD<'a, T>>>,
// }
