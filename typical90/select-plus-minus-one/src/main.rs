use ndarray;
use ndarray::Array1;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[isize; n],
        b:[isize; n]
    }
    let a = Array1::from_shape_vec(n, a);
    let b = Array1::from_shape_vec(n, b);
    print!("{:#?}", &a - &b);

    // println!("{}", b - a);
}
