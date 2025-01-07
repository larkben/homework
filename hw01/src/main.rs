use problem1::sum;

mod problem1;

fn main() {
    println!("Hello, world!");

    let temp: [i32; 4] = [1,2,3,4];

    println!("test: {}", sum(&temp));
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
