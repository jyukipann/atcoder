use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    }
    println!("{}", a.iter().filter(|&x| x >= &l).count());
}
