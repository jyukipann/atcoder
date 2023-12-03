use proconio::input;

fn main() {
    // 
    input! {
        month: isize,
        day: isize,
        y: isize,
        m: isize,
        d: isize,
    }
    let nd = if d+1 > day { 1 } else { d+1 };
    let nm = if d+1 > day {
        if m+1 > month { 1 } else { m+1 }
    }else{
        m
    };
    let ny = if m+1 > month { y+1 } else { y };
    print!("{} {} {}", ny, nm, nd);
}
