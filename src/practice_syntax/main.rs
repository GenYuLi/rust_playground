use rust_playground::practice_syntax::loop_test;
use regex::Regex;
use rust_playground::practice_syntax::learn_enum;
use rust_playground::practice_syntax::try_vec;


fn main() {
    //let my_ip = learn_enum::IpAddrKind::V4(127, 0, 0, 1);
    let my_ip = learn_enum::IpAddrKind::V6(String::from("::1"));
    if let learn_enum::IpAddrKind::V4(a, b, c, d) = my_ip {
        println!("{} {} {} {}", a, b, c, d);
    } else if let learn_enum::IpAddrKind::V6(ip) = my_ip {
        println!("{}", ip);
    }
    loop_test::loop_test();
    let re = Regex::new(r"^[A-CEGHJ-PRSTW-Z][A-CEGHJ-NPRSTW-Z]\d{6}[ABCD]$").unwrap();
    let ni_number = "QQ123456C"; // 測試用的例子
    if re.is_match(ni_number) {
        println!("有效的國民保險號碼");
    } else {
        println!("無效的國民保險號碼");
    }
}