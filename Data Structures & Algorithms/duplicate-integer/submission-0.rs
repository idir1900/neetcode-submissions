impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                if nums[i]==nums[j] {
                    return true;
                }
            }
        }
        return false;
    }
}
