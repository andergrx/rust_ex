use std::rc::Rc;
use std::cell::RefCell;

fn main() -> () {
    

    // let mut a = vec![];
    // a.push(1);

    // println!("{}", a[0]);

    // //let arc = Arc::new();

    // let ref_cell = RefCell::new(1);
    // println!("refc: {:?}", ref_cell);
    // println!("refc: {:?}", ref_cell.borrow());

    // let b1 = ref_cell.borrow_mut();
    // let b2 = ref_cell.borrow_mut();
    // println!("refc: {:?}", b1);
    // println!("refc: {:?}", b2);

    let a = Rc::new(RefCell::new(42));
    let b = a.clone();
    *b.borrow_mut() += 1;
    let c = b.clone();
    *c.borrow_mut() += 1;
    //let a = 42;
    // let a = Box::new(42);
    // let b = Box::clone(&a);
    println!("a: {:?}, b: {}", a, b.borrow());
    println!("{:p}, {:p}, {:p}, {:p}", &a, &*a, &b, &*b);

}
