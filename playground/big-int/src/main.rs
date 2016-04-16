use std::io::prelude::*;

#[derive(Debug)]
pub struct BigInt {
    pub data: Vec<u64>,
}

pub trait Minimum{
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self;
}

impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }
    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    pub fn from_vec(mut v: Vec<u64>) -> Self {
        loop {
            match v.last() {
                Some(&i) => {
                    if i == 0 {
                        v.pop();
                        continue;
                    }
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        BigInt { data: v }
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt { data: self.data.clone() }
    }
}

impl BigInt {
    pub fn min_try1<'a>(&'a self, other: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            self
        }
    }


    pub fn vec_min<T: Minimum>(v: &Vec<T>) -> Option<&T> {
        let mut min: Option<&T> = None;
        for e in v {
            min = match min {
                None => Some(e),
                Some(n) => Some(n.min(e)),
            };
        }
        min
    }

    pub fn head<T>(v: &Vec<T>) -> Option<&T> {
        if v.len() > 0 {
            Some(&v[0])
        } else {
            None
        }
    }
}

impl Minimum for BigInt {
    fn min<'a>(&'a self, other: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant() && other.test_invariant());
        if self.data.len() < other.data.len() {
            self
        } else if self.data.len() > other.data.len() {
            other
        } else {
            self
        }
    }
}
pub fn main() {
    let v1 = vec![1, 0, 1 << 16, 0, 0, 0, 0];
    let v2 = vec![1, 0, 234];
    let b1 = BigInt::from_vec((&v1).clone());
    println!("{:?}", b1);
    println!("{:?}", b1.test_invariant());
    let b2 = BigInt::from_vec((&v2).clone());
    println!("{:?}", b2.test_invariant());
    println!("{:?}", BigInt::head(&v1).unwrap());
    let bv: Vec<BigInt> = vec![b1, b2];
    println!("{:?}", BigInt::vec_min(&bv).unwrap());
}
