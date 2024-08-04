
pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let mut new_nums = Vec::new();
    for idx in 0..nums.len() {
        let mut sum = 0;
        for j in idx..nums.len() {
            sum += nums[j];
            new_nums.push(sum);
        }
    }
    println!("{:?}", new_nums);
    new_nums.sort();
    println!("{:?}", new_nums);
    let mut ans = 0;
    for i in left-1..=right-1 {
        ans += new_nums[i as usize];
    }
    ans
}


pub fn heap_range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut pq = BinaryHeap::new();

    for i in 0..n {
        pq.push(Reverse((nums[i] as i64, i, i)));
    }

    let mut result: i64 = 0;

    for i in 1..=right as usize {
        if let Some(Reverse((sum, start, end))) = pq.pop() {
            if i>= left as usize {
                result = (result + sum) % MOD;
            }
            if end < n-1 {
                pq.push(Reverse((sum + nums[end + 1] as i64, start, end + 1)));
            }
        }
    }

    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_sum() {
        let nums = vec![1, 2, 3, 4];
        let n = 4;
        let left = 1;
        let right = 5;
        let result = range_sum(nums, n, left, right);
        assert_eq!(result, 13);
    }
}