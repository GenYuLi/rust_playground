use std::collections::{HashMap, BinaryHeap, LinkedList};
use std::cmp::Reverse;

struct KthLargest {
    pub heap: BinaryHeap<Reverse<i32>>,
    kth: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut instance = KthLargest{
            heap: BinaryHeap::new(),
            kth: k,
        };
        for num in nums {
            instance.add(num);
        }
        instance
    }

    
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if (self.heap.len() as i32) > self.kth {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}


pub fn minimum_pushes(word: String) -> i32 {
    let mut arr = [0; 26];
    word.chars().for_each(|c| {
        arr[(c as u8 - 'a' as u8) as usize] += 1;
    });
    arr.sort_unstable_by(|a, b| b.cmp(a));
    let length = arr.len();
    let mut ans = 0;
    for i in 0..length {
        let multiply = (i as i32 / 8) + 1;
        ans += arr[i]*multiply;
    }
    ans
}

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

pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut distinct = HashMap::new();
    let mut times = 0;
    arr.iter().for_each(|s| {
        *distinct.entry(s).or_insert(0) += 1;
    });
    println!("{:?}", distinct);
    for s in arr.iter() {
        if *distinct.get(s).unwrap() == 1 {
            times += 1;
            if times == k {
                return s.to_string();
            }
        }
    }
    "".to_string()
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
    fn test_kth_largest() {
        let k = 3;
        let nums = vec![4, 5, 8, 2];
        let mut kth_largest = KthLargest::new(k, nums);
        let result = kth_largest.add(3);
        assert_eq!(result, 4);
        let result = kth_largest.add(5);
        assert_eq!(result, 5);
        let result = kth_largest.add(10);
        assert_eq!(result, 5);
        let result = kth_largest.add(9);
        assert_eq!(result, 8);
        let result = kth_largest.add(4);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_range_sum() {
        let nums = vec![1, 2, 3, 4];
        let n = 4;
        let left = 1;
        let right = 5;
        let result = range_sum(nums, n, left, right);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_minimum_pushes() {
        let word = "aaabbbcc".to_string();
        let result = minimum_pushes(word);
        assert_eq!(result, 8);
        let word = "aabbccddeeffgghhiiiiii".to_string();
        let result = minimum_pushes(word);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_kth_distinct() {
        let arr = vec!["a".to_string(), "b".to_string(), "c".to_string(), "a".to_string(), "c".to_string()];
        let k = 1;
        let result = kth_distinct(arr, k);
        assert_eq!(result, "b");
    }
}