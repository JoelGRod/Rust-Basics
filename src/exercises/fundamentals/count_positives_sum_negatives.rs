/*
    Given an array of integers.
    
    Return an array, where the first element is the count of positives numbers 
    and the second element is sum of negative numbers. 0 is neither positive 
    nor negative.
    
    If the input is an empty array or is null, return an empty array.
    
    Example
    For input [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], 
    you should return [10, -65].
*/

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut results: Vec<i32> = vec![0, 0];

    for num in &input {
        if num > &0 {
            results[0] += 1
        } else {
            results[1] += num
        };
    }

    if results[0] == 0 && results[1] == 0 {
        vec![]
    } else {
        results
    }
}


fn count_positives_sum_negatives_two(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    input.iter().fold(vec![0, 0], |mut acc, &x| {
        if x > 0 {
            acc[0] += 1;
        } else {
            acc[1] += x;
        }
        acc
    })
}


fn count_positives_sum_negatives_three(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }
    let count_positives = input.iter().filter(|&&x| x.is_positive()).count() as i32;
    let sum_negatives = input.iter().filter(|&&x| x.is_negative()).sum();
    vec![count_positives, sum_negatives]
}


fn count_positives_sum_negatives_four(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }

    vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let test_data2 = vec![0, 0];
        let test_data3 = vec![1, 2, 3, 4, 5, 6, 7, 0, 9, 10, 0, -12, -13, -14, -15];
        let expected1 = vec![10, -65];
        let expected3 = vec![9, -54];
        assert_eq!(count_positives_sum_negatives(test_data1), expected1);
        assert!(!count_positives_sum_negatives_two(test_data2).is_empty());
        assert_eq!(count_positives_sum_negatives(test_data3), expected3);
    }

    #[test]
    fn returns_expected_two() {
        let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let test_data2 = vec![0, 0];
        let test_data3 = vec![1, 2, 3, 4, 5, 6, 7, 0, 9, 10, 0, -12, -13, -14, -15];
        let expected1 = vec![10, -65];
        let expected3 = vec![9, -54];
        assert_eq!(count_positives_sum_negatives_two(test_data1), expected1);
        assert!(!count_positives_sum_negatives_two(test_data2).is_empty());
        assert_eq!(count_positives_sum_negatives_two(test_data3), expected3);
    }

    #[test]
    fn returns_expected_three() {
        let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let test_data2 = vec![0, 0];
        let test_data3 = vec![1, 2, 3, 4, 5, 6, 7, 0, 9, 10, 0, -12, -13, -14, -15];
        let expected1 = vec![10, -65];
        let expected3 = vec![9, -54];
        assert_eq!(count_positives_sum_negatives_three(test_data1), expected1);
        assert!(!count_positives_sum_negatives_two(test_data2).is_empty());
        assert_eq!(count_positives_sum_negatives_three(test_data3), expected3);
    }

    #[test]
    fn returns_expected_four() {
        let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
        let test_data2 = vec![0, 0];
        let test_data3 = vec![1, 2, 3, 4, 5, 6, 7, 0, 9, 10, 0, -12, -13, -14, -15];
        let expected1 = vec![10, -65];
        let expected3 = vec![9, -54];
        assert_eq!(count_positives_sum_negatives_four(test_data1), expected1);
        assert!(!count_positives_sum_negatives_two(test_data2).is_empty());
        assert_eq!(count_positives_sum_negatives_four(test_data3), expected3);
    }
}
