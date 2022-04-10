/*
    Your task is to find the first element of an array that is not consecutive.
    
    By not consecutive we mean not exactly 1 larger than the previous element
    of the array.
    
    E.g. If we have an array [1,2,3,4,6,7,8] then 1 then 2 then 3 then 4 are
    all consecutive but 6 is not, so that's the first non-consecutive number.
    
    If the whole array is consecutive then return null2.
    
    The array will always have at least 2 elements1 and all elements will be
    numbers. The numbers will also all be unique and in ascending order.
    The numbers could be positive or negative and the first non-consecutive
    could be either too!
*/

fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    // code here
    let mut checker: i32 = arr[0];

    for digit in arr {
        if digit == &checker {
            checker += 1
        } else {
            return Some(*digit);
        }
    }

    None
}

fn first_non_consecutive_two(arr: &Vec<i32>) -> Option<i32> {
    for (idx, digit) in arr.iter().enumerate().skip(1) {
        if digit - 1 != arr[idx - 1] {
            return Some(*digit);
        }
    }
    None
}

fn first_non_consecutive_three(arr: &Vec<i32>) -> Option<i32> {
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] != 1 {
            return Some(arr[i]);
        }
    }
    None
}

fn first_non_consecutive_four(arr: &Vec<i32>) -> Option<i32> {
    for window in arr.windows(2) {
        if window[1] - 1 != window[0] {
            return Some(window[1]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 6, 7, 8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1, 2, 3, 4, 5, 6, 7, 8]), None);
        assert_eq!(first_non_consecutive(&vec![4, 6, 7, 8, 9, 11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4, 5, 6, 7, 8, 9, 11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive(&vec![-5, -4, -3, -1]), Some(-1));
    }

    #[test]
    fn test_basic_two() {
        assert_eq!(
            first_non_consecutive_two(&vec![1, 2, 3, 4, 6, 7, 8]),
            Some(6)
        );
        assert_eq!(
            first_non_consecutive_two(&vec![1, 2, 3, 4, 5, 6, 7, 8]),
            None
        );
        assert_eq!(first_non_consecutive_two(&vec![4, 6, 7, 8, 9, 11]), Some(6));
        assert_eq!(
            first_non_consecutive_two(&vec![4, 5, 6, 7, 8, 9, 11]),
            Some(11)
        );
        assert_eq!(first_non_consecutive_two(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive_two(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive_two(&vec![-5, -4, -3, -1]), Some(-1));
    }

    #[test]
    fn test_basic_three() {
        assert_eq!(
            first_non_consecutive_three(&vec![1, 2, 3, 4, 6, 7, 8]),
            Some(6)
        );
        assert_eq!(
            first_non_consecutive_three(&vec![1, 2, 3, 4, 5, 6, 7, 8]),
            None
        );
        assert_eq!(
            first_non_consecutive_three(&vec![4, 6, 7, 8, 9, 11]),
            Some(6)
        );
        assert_eq!(
            first_non_consecutive_three(&vec![4, 5, 6, 7, 8, 9, 11]),
            Some(11)
        );
        assert_eq!(first_non_consecutive_three(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive_three(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive_three(&vec![-5, -4, -3, -1]), Some(-1));
    }

    #[test]
    fn test_basic_four() {
        assert_eq!(
            first_non_consecutive_four(&vec![1, 2, 3, 4, 6, 7, 8]),
            Some(6)
        );
        assert_eq!(
            first_non_consecutive_four(&vec![1, 2, 3, 4, 5, 6, 7, 8]),
            None
        );
        assert_eq!(
            first_non_consecutive_four(&vec![4, 6, 7, 8, 9, 11]),
            Some(6)
        );
        assert_eq!(
            first_non_consecutive_four(&vec![4, 5, 6, 7, 8, 9, 11]),
            Some(11)
        );
        assert_eq!(first_non_consecutive_four(&vec![31, 32]), None);
        assert_eq!(first_non_consecutive_four(&vec![-3, -2, 0, 1]), Some(0));
        assert_eq!(first_non_consecutive_four(&vec![-5, -4, -3, -1]), Some(-1));
    }
}
