use float_ord::FloatOrd;
use std::{fs::File, io::{BufWriter, Write}};

fn main() {
    let values = [1, 4, 7, 9, 11, 20, 33];
    let li = get_low_index(6, &values);
    let hi = get_high_index(6, &values);
    println!("values: {:?}, {:?}, {:?}, {:?}", li, hi, get_low_value(6, &values), get_high_value(6, &values));
    let li = get_low_index(0, &values);
    let hi = get_high_index(0, &values);
    println!("values: {:?}, {:?}, {:?}, {:?}", li, hi, get_low_value(0, &values), get_high_value(0, &values));
    let li = get_low_index(99, &values);
    let hi = get_high_index(99, &values);
    println!("values: {:?}, {:?}, {:?}, {:?}", li, hi, get_low_value(99, &values), get_high_value(99, &values));

    let v = vec![1, 3, 7, 19, 22, 33, 44];
    let vi = v.binary_search(&6);
    println!("vec res: {:?}", vi);

    let vr = &v;
    let vri = vr.binary_search(&19);
    println!("vec res: {:?}", vri);

    let vf = vec![1.1, 2.56, 8.9, 11.55, 56.2156, 87.9];
    let vfi = vf.binary_search_by_key(&FloatOrd(5.5), |&x| FloatOrd(x));
    println!("vec res: {:?}", vfi);

    let fb: f32 = 3.14567;
    println!("float bits: 0x{:.8x}", fb.to_bits());

    let write_file = File::create("./tmp.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);
    _ = write!(&mut writer, "float bits: {:.6o} 0x{:.8x}\n", fb.to_bits(), fb.to_bits());

    let ud = -5;
    _ = write!(&mut writer, "udata: {:#.6o} {:#.4x}\n", ud as u16, ud as u16);
    println!("udata: {:#.6o} {:#.4x}\n", ud as u16, ud as u16);

}

fn get_low_value(val: i32, input: &[i32]) -> i32 {
    input[get_low_index(val, input)]
}

fn get_high_value(val: i32, input: &[i32]) -> i32 {
    input[get_high_index(val, input)]
}

fn get_low_index(val: i32, input: &[i32]) -> usize {
    let index = input.binary_search(&val);
    match index {
        Ok(i) => i,
        Err(i) => {
            if i <= 0 {
                i
            } else {
                i - 1
            }
        }
    }
}

fn get_high_index(val: i32, input: &[i32]) -> usize {
    let index = input.binary_search(&val);
    match index {
        Ok(i) => {
            if i + 1 >= input.len() {
                i
            } else {
                i + 1
            }
        }
        Err(i) => {
            if i >= input.len() {
                i - 1
            } else {
                i
            }
        }
    }
}
