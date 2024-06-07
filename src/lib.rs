pub mod assignment_1;
pub mod assignment_2;
pub mod assignment_3;

#[cfg(test)]
mod tests {
    use crate::assignment_1::assignment_1;
    use crate::assignment_2::assignment_2;
    use crate::assignment_3::assignment_3;

    // Strictly Increasing
    #[test]
    fn test_assignment_1() {
        assert_eq!(assignment_1::resulting(vec![1, 2, 10, 5, 7]), true);
        assert_eq!(assignment_1::resulting(vec![2, 3, 1, 2]), false);
        assert_eq!(assignment_1::resulting(vec![1, 1, 1]), false);
    }

    // Roman Numerals
    #[test]
    fn test_assignment_2() {
        assert_eq!(assignment_2::resulting("III"), 3);
        assert_eq!(assignment_2::resulting("LVIII"), 58);
        assert_eq!(assignment_2::resulting("MCMXCIV"), 1994);
    }

    // Bulb Switcher
    #[test]
    fn test_assignment_3() {
        assert_eq!(assignment_3::resulting(&mut vec![2, 1, 3, 5, 4]), 3);
        assert_eq!(assignment_3::resulting(&mut vec![3, 2, 4, 1, 5]), 2);
        assert_eq!(assignment_3::resulting(&mut vec![5, 1, 2, 3, 4]), 1);
        assert_eq!(assignment_3::resulting(&mut vec![2, 1, 4, 3, 5]), 3);
        assert_eq!(assignment_3::resulting(&mut vec![1, 2, 3, 4, 5]), 5);
    }
}