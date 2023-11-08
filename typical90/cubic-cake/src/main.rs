use proconio::input;
use num::{self, integer::gcd};

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize
    }
    let gcd_num = num::integer::gcd(a,b);
    let gcd_num = num::integer::gcd(gcd_num,c);
    let mut cut_count: usize = 0;
    cut_count += (a / gcd_num)-1;
    cut_count += (b / gcd_num)-1;
    cut_count += (c / gcd_num)-1;
    print!("{}", cut_count);
}
