use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                // If the complement exists in the map, it means we have found the two numbers that add up to the target
                return vec![index as i32, i as i32];
            }
            
            // If the complement is not in the map, insert the current number with its index into the map
            // This way, we can find it later if needed
            map.insert(num, i);
        }
        
        vec![] // Default case, technically should not be reachable
    }
}
