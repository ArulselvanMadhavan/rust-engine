#![cfg(test)]

use problem1::{sum, dedup, filter};


#[test]
fn test_sum_large() {
    let array = [4294967296,1];
    let result = sum(&array);
    println!("{}", result);
    assert_eq!(result, 4294967297);
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
