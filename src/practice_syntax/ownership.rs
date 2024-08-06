
pub fn test_ownership() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // error[E0382]: borrow of moved value: `s1`
    test_take_ownership(s2);
    //println!("{}", s2); // error[E0382]: borrow of moved value: `s2`
}

pub fn test_take_ownership(taking_ownership: String) {
    println!("{}", taking_ownership);
}