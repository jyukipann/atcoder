use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let ans: usize = a.iter().sum();
    println!("{}", ans%100);
}
