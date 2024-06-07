#[cfg(test)]
pub mod assignment_2 {
    use std::collections::HashMap;

    pub fn resulting(roman_nums: &str) -> i32 {
        let roman_map: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
    
        let mut nums = vec![];
    
        for r in roman_nums.chars() {
            let num = match roman_map.get(&r) {
                Some(x) => x,
                None => panic!("Error: key not found"),
            };
            nums.push(num);
        }
        nums.push(&0);
    
        let mut result = 0;
    
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] {
                result -= nums[i]
            } else {
                result += nums[i]
            }
        }
    
        result
    }
}