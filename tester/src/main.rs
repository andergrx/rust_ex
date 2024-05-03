struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?})", self.raw, self.digits)
    }
}

fn main() {
    let s = "978-0-545-01022-1";
    let v: Vec<u8> = s.replace("-","").bytes().map(|x| x-0x30).collect();
    println!("digits: {:?}", v);
    
    let isbn = Isbn {
        raw: s.to_string(),
        digits: v,
    }; 
    println!("ISBN: {}", isbn);

    if is_valid(&isbn) {
        println!("ISBN valid!");
    } else {
        println!("ISBN NOT valid!");
    }


}

fn is_valid(isbn: &Isbn) -> bool {

    let mut my_digs = isbn.digits.clone();
    let last = my_digs.pop().expect("no digits");
    println!("last: {:?}, remaining vec: {:?}", last, my_digs);

    let alternate_x_1: Vec<u8> = my_digs.iter().step_by(2).map(|x| *x).collect();
    let alternate_x_3: Vec<u8> = my_digs.iter().skip(1).step_by(2).map(|x| *x*3).collect();
    println!("alt vecs: {:?}, {:?}", alternate_x_1, alternate_x_3);

    let sum = alternate_x_1.iter().sum::<u8>() + alternate_x_3.iter().sum::<u8>();
    let mut check = 10 - sum%10;
    if check == 10 {
        check = 0;
    }
    println!("sum: {:?}, {:?}", sum, check); 

    check == last 
}