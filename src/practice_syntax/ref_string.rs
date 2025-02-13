
pub fn ref_test() {
    let mut str = String::from("3434");
    str.push_str("324234");
    let type_name = &str[0..3];
    let test = &str[0..3];
    let r1 = &mut str;
    let r2: &mut String = &mut str;
    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. 
    println!("{r2}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}

pub fn ref_clear() {
    let mut s = "hello world";

    let mut s1 = String::from("hello ");
    let mut s2 = String::from("world");
    let s3 = s1 + &s2;
}

pub fn dangle(str1:&String) -> &String {
    let str2 = String::from("hello");
    //str2 // -> compiler error
    str1 // -> OK
}