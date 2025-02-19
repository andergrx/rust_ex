
#[allow(unused_imports)]
use ndarray::prelude::*;

fn main() {

    let matrix =
        array![[1,2,3,4,5],
               [6,7,8,9,10],
               [11,12,13,14,15],
               [16,17,18,19,20]] * 4;

    println!("matrix: {matrix:?}");

    println!("elem (2,3): {}", matrix[[2,3]]);
}
