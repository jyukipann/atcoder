use proconio::input;

fn main() {
    input! {
        month: isize,
        day: isize,
        y: isize,
        m: isize,
        d: isize,
    }
    let (ny, nm, nd) = if m == month && d == day {
        (y+1, 1, 1)
    } else if d == day {
        (y, m+1, 1)
    } else {
        (y, m, d+1)
    };
    print!("{} {} {}", ny, nm, nd);
}
