use proconio::input;

fn main() {
    input! {
        n:usize,
        x:usize,
        a:[usize; n],
    }
    let mut is_in = false;
    for ai in a {
        if x == ai {
            is_in = true;
            break;
        }
    }
    if is_in { print!("Yes") } else { print!("No") };
}
