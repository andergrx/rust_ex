use std::rc::Rc;

use linked_list::llist::{Node, NodeD};

fn main() {
    
    let mut ll_1 = Node {
        data: 42,
        next: None,
    };

    let ll_2 = Node {
        data: 43,
        next: None,
    };

    ll_1.next = Some(Box::new(ll_2));


    println!("{:?}", ll_1);


    let dl = NodeD::new_ptr(4.2);

    let dl_2 = NodeD::new_ptr_with_prev(4.3, dl.clone());

    let dl_3 = NodeD::new_ptr(4.4);

    //println!("{:p}, {:p}", &dl.borrow().data, &dl.clone().borrow().data);

    dl.borrow_mut().next = Some(Rc::clone(&dl_2));
    //dl_2.borrow_mut().prev = Some(dl.clone());
    dl_3.borrow_mut().prev = Some(dl_2.clone());

    println!("{:p}, {:p}", &dl.borrow().data, &NodeD::get_prev(&dl_2).borrow().data);
    println!("{:p}, {}", &NodeD::get_prev(&dl_3).borrow().data, &NodeD::get_prev(&dl_3).borrow().data);
    println!("{:p}. {}", &dl_2.borrow().data, dl_2.borrow().data);
    println!("{:p}. {:p}", &dl_2.borrow().data, &NodeD::get_next(&dl).borrow().data);


}
