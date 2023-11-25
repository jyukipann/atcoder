use proconio::input;

fn main() {
    input! {
        n : usize,
        p : [usize; n],
    }
    let coffient: Vec<f64> = (0..n).map(|k| (0.9_f64).powf(k as f64)).collect();
    let mut dp_table = vec![vec![0_f64; n]; n];
    let mut max_v = 0_f64;
    for i in 0..n{
        for j in 0..n{
            dp_table[i][j] = coffient[n-1-i]*(p[j] as f64) + max_v;
        }
        max_v = dp_table[i];
        
    }

    println!("{}", 0);
}
