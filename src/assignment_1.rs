#[cfg(test)]
pub mod assignment_1 {
    pub fn resulting(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            let new_nums = removing(i, nums.clone());
            if increase_checking(new_nums) {
                return true;
            }
        }

        false
    }
    
    fn removing(index: usize, nums: Vec<i32>) -> Vec<i32> {
        let mut nums_copy = nums.clone();
        nums_copy.remove(index);

        nums_copy
    }

    fn increase_checking(nums: Vec<i32>) -> bool {
        let mut result: bool = false;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                result = true
            } else {
                return false;
            }
        }

        result
    }
}
