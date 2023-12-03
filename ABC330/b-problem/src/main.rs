use proconio::input;
// ∣Xi​−Ai​∣≤∣Y−Ai​∣
fn main() {
    input! {
        n: usize,
        l: isize,
        r: isize,
        a: [isize; n],
    }
    let mut x = vec![0_isize; n];
    for i in 0..n {
        let min_of_y_ai = match (l..=r).map(|y| ((y-a[i])).abs()).min() {
            Some(v) => v,
            None => {println!("min_of_y_ai None"); 0},
        };
        x[i] = match (l..=r).map(|x| x-a[i]).filter(|&x| -min_of_y_ai <= x && x <= min_of_y_ai).next() {
                Some(v) => v+a[i],
                None => {println!("x[i] None"); 0},
        };
        print!("{} ", x[i]);
    }
}
