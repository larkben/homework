use problem1::{sum, dedup};

mod problem1;

fn main() {
    println!("Hello, world!");

    let temp: [i32; 4] = [1,2,3,4];
    let mut dep = Vec::new();

    dep.push(1);
    dep.push(1);
    dep.push(2);

    println!("test: {}", sum(&temp));

    let dedup = dedup(&dep);

    println!("test dedup: {}, {}", dedup[0], dedup[1]);
}

/*
this becomes unweildy and nasty so instead we make seperate .rs files in /src

pub mod problem1 {

}

pub mod problem2 {

}

pub mod problem3 {

}

pub mod problem4 {

}
*/
