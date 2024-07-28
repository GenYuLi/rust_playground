fn score_of_string(s: String) -> i32 {
        let mut sum : i32 = 0;
        let mut chars = s.chars();
        let mut left_char = chars.next().unwrap() as i32;
        for c in chars {
            sum += (left_char - c as i32).abs();
            left_char = c as i32;
        }
        return sum;
}

fn main() {
    let s = String::from("hello");
    println!("answer:{}", score_of_string(s));
}