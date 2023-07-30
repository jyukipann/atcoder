use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
    }
    let n_mod_x:usize = n / x ;
    let n_mod_y:usize = n / y ;
    let n_mod_xy:usize = n / (x * y);
    let ans = n_mod_x + n_mod_y - n_mod_xy;
    println!("{}", ans);
}
