use std::vec::Vec;

pub fn try_vec() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for num in &mut v {
        *num += 50;
        println!("{}", num);
    }
}