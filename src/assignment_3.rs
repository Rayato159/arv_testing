#[cfg(test)]
pub mod assignment_3 {
    pub fn resulting(orders: &mut Vec<i32>) -> i32 {
        let mut counting_result = 0;

        for i in 0..orders.len() {
            let current_bulb = orders[i].abs();

            orders[current_bulb as usize - 1] = orders[current_bulb as usize - 1] * -1;

            let mut on_bulb = 0;

            for j in 0..i + 1 {
                if orders[j] < 0 {
                    on_bulb += 1
                }
            }

            if on_bulb == i + 1 {
                counting_result += 1
            }
        }

        counting_result
    }
}
