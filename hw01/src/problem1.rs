// problem one

// pub ~ makes this globally available; when normally it is private to this file

//use std::iter;

pub fn sum(slice: &[i32]) -> i32 {
    let mut temp = 0;
    for i in slice {
        temp = temp + i
    };
    temp
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut dedup = Vec::new();

    for i in vs {
        if !dedup.contains(i) {
            dedup.push(*i);
        }
    }
    dedup
}

pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
    let mut temp_vec = Vec::new();

    for i in vs {
        //println!("{}", i);
        if pred(*i) {
            temp_vec.push(*i)
        }
        
    }
    temp_vec
}

