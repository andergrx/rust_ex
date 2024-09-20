use core::fmt;

fn main() {
    let a = 5;
    pt(&a);

    let mut v = Vec::new();
    v.push(4.5);
    pt(&v);

    let b = [3, 4, 5];
    pt(&b);

    let t = (1, "gabe", 7.7);
    pt(&t);

    let z = &t;
    pt(&z);

    let k = t;
    pt(&k);

    {
        let vv = &mut v;
        pt(&vv);

        vv.push(9.9);
        println!("vv: {:?}", vv);
    }
    v.push(1.1);

    println!("v: {:?}", v);

    let arr: [u8; 5] = [42; 5];
    pt(&arr);

    println!("arr: {:?}", arr);

    let c = "123absdkjafh".chars();
    let d: Vec<_> = c.clone().collect();
    pt(&c);
    pt(&d);

    let cc = "name".bytes().map(|x| x + 1);
    let ccv: Vec<_> = cc.clone().collect();
    pt(&cc);
    pt(&ccv);

    println!("pre cc: {:?}", "name".bytes().collect::<Vec<_>>());
    println!("ccv: {:?}", ccv);

    pt(&pt::<String>);

    pt(&v.iter());

    let mut m = vec![];
    m.push("kkkk".to_string());
    pt(&m);

    let ms = MyStruct { val: 5, v_ref: &0 };
    println!("ms: {}", ms);

    let mut p: *mut Vec<u32> = std::ptr::null_mut();
    p = &mut Vec::<u32>::new();
    unsafe {
        (*p).push(44);

        println!("P: {:?}", *p);
    }
    println!("P: {:?}", p);

    let pp = &mut Vec::<u64>::new();
    pp.push(99);
    println!("PP: {:?}", pp);

    let mut x: &mut u32;
    let mut y: u32 = 42;
    
    x = &mut y;
    *x += 1;
    //y += 1; //cannnot do this
    println!("X: {:?}", x);

    let get_evens = | input: Vec<u32> | -> Vec<u32> {

        println!("input addr: {:p}, {:p}", &input, &input[0]);
        input.into_iter().filter(|x| x%2 == 0).collect()
    };

    let tv: Vec<u32> = vec![11,22,33,44,55,66,77,88];
    println!("tv addr: {:p}, {:p}", &tv, &tv[0]);
    let evens = get_evens(tv);
    println!("Evens vec: {:?}", evens);
    println!("evens addr: {:p}, {:p}", &evens, &evens[0]);

    let mut ptr: *mut Vec<u32> = std::ptr::null_mut();
    ptr = &mut Vec::<u32>::new();
    unsafe {
        (*ptr).push(45);
        println!("ptr: {:?}", *ptr);
    };

    let signature = {
        // Combine this data together and SCALE encode it:
        let a = (1, 4.5, "hello");
        let b = String::from("Gabe");

        // If payload is longer than 256 bytes, we hash it and sign the hash instead:
        if a.1  > 10. {
            println!("if");
        } else {
            println!("else");
        }
    };

    let thing = {
        let a = 1;
        let b = 4;
        println!("I'm gonna return 3");
        3
    };

    println!("Thing {}", thing);

}

struct MyStruct<'a> {
    val: i32,
    v_ref: &'a u32,
}

impl std::fmt::Display for MyStruct<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.val, self.v_ref)
    }
}

fn pt<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>())
}

//fn life(first: &u32, second: &u32 ){

// }
