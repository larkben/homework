// problem one

// pub ~ makes this globally available; when normally it is private to this file

use std::iter;

pub fn sum(slice: &[i32]) -> i32 {
    let mut temp = 0;
    for i in slice {
        temp = temp + i
    };
    temp
}

#[warn(unused_variables)]
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut first: Vec<i32> = Vec::new();
    
    // add to first instance
    for i in vs {
        // check if duplicate
        for p in first {
            if i == p {
                break
            }
        }
    }
    first
}

#[warn(unused_variables)]
pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    // todo
    unimplemented!()
}

