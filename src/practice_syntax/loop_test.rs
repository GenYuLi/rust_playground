pub fn loop_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        let mut tmp_s = "3434";
        if tmp_s == "3434" {
            println!("tmp_s = {tmp_s}");
        }
        tmp_s = "2";
        let tmp = tmp_s.parse::<i32>().unwrap();
        println!("{tmp}");
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 1 {
                count += 1;
                continue 'counting_up;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}
