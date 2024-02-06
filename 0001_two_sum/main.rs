impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut a: Vec<i32> = vec![];
        let m_h: HashMap<_, _> = nums
            .iter()
            .enumerate()
            .map(|(index, &value)| (value, index))
            .collect();

        for (nums_i, nums_v) in nums.iter().enumerate() {
            let o = &target - nums_v;
            match m_h.get(&o) {
                Some(&ii) => {
                    if ii == nums_i {
                        continue;
                    }
                    a.push(nums_i as i32);
                    a.push(ii as i32);
                    break;
                }
                None => {
                    continue;
                }
            }
        }
        a
    }
    // more efficient
    pub fn two_sum_other(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&other) = map.get(&(target - num)) {
                return vec![other as i32, i as i32];
            }
            map.insert(num, i);
        }

        panic!()
    }
}
