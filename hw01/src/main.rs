use problem1::{sum, dedup, filter};
use problem2::{mat_mult};

mod problem1;
mod problem2;

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

    // filter ~ need to understand this second param more
    filter(&dep, &|x| x % 3 == 0);

    let matrix_one: Vec<Vec<f32>> = vec![vec![0.1, 0.3], vec![0.3, 0.5]];
    let matrix_two: Vec<Vec<f32>> = vec![vec![0.1, 0.2], vec![0.3, 0.4]];

    mat_mult(&matrix_one, &matrix_two);
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
