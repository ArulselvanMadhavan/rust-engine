use std::io::prelude::*;

#[derive(Debug)]
pub struct BigInt {
    pub data: Vec<u64>,
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

impl BigInt{
    pub fn min_try1(self,other:Self)->Self{
        debug_assert!(self.test_invariant() && other.test_invariant());
    }
}
pub fn main() {
    let v = vec![1,0, 1 << 16,0,0,0,0];
    let b1 = BigInt::from_vec((&v).clone());
    println!("{:?}", b1);
    println!("{:?}", b1.test_invariant());
    let b2 = BigInt::from_vec(v);
    println!("{:?}", b2.test_invariant());
}
