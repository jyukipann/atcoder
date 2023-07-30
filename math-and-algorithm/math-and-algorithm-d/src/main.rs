use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    }
    let product: usize = a.iter().fold(1, |acc, x| acc*x);
    println!("{}", product);
}
