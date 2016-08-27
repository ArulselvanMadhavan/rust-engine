#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::mat_mult;

#[test]
fn test_sum_large() {
    let array = [4294967294,1];
    let result = sum(&array);
    println!("{}", result);
    assert_eq!(result, 4294967295);
}

#[test]
fn test_sum_single() {
    let array = [24234];
    assert_eq!(sum(&array), 24234);
}

#[test]
fn test_sum_empty() {
    let array = [];
    assert_eq!(sum(&array), 0);
}

#[test]
fn test_dedup_all() {
    let vs = vec![1,1,1,1,1,1];
    assert_eq!(dedup(&vs), vec![1]);
}

#[test]
fn test_dedup_empty() {
    let vs = vec![];
    assert_eq!(dedup(&vs), vec![]);
}

fn odd_predicate(num: i32) -> bool{
    (num % 2) == 1
}

#[test]
fn test_filter_all() {
    let vs = vec![2,4,6,8,10];
    assert_eq!(filter(&vs, &odd_predicate), vec![]);
}

#[test]
fn test_matrix_multiply_large() {
    let mat1 = vec![vec![3., 2., 1. ], vec![4.,5.,6.], vec![7.,8.,9.], vec![10.,11.,12.]];
    let mat2 = vec![vec![1.,1.], vec![11.,11.], vec![111.,111.]];
    let exp_result = vec![vec![136., 136.], vec![725.,725.], vec![1094., 1094.], vec![1463., 1463.]];
    let act_result = mat_mult(&mat1, &mat2);
    assert!(exp_result.len() == act_result.len());
    assert!(exp_result[0].len() == act_result[0].len());
    for row in 0..act_result.len() {
        for col in 0..act_result[0].len() {
            assert_eq!(act_result[row][col], exp_result[row][col]);
        }
    }
}
