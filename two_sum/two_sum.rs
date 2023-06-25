use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_num: HashMap<i32, i32> = HashMap::new();

        for(i, &num) in nums.iter().enumerate() {
            let substract = target - num;
            if let Some(&index) = hash_num.get(&substract) {
                return vec![i as i32, index as i32];
            }
            hash_num.insert(num, i as i32);
        }

        vec![]
    }
}