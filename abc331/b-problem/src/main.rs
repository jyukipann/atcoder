use proconio::input;

fn main() {
    input! {
        n : isize,
        s_price : isize,
        m_price : isize,
        l_price : isize,
    }
    let (s_count, m_count, l_count) = (6, 8, 12);
    let mut min_price = std::isize::MAX;
    for s in 0..=((n as f64/s_count as f64).ceil() as isize) {
        for m in 0..=((n as f64/m_count as f64).ceil() as isize) {
            let amount_left = n - (s*s_count + m*m_count);
            let l = if amount_left < 0 {
                0
            } else {
                (amount_left as f64 / l_count as f64).ceil() as isize
            };
            let price = s*s_price + m*m_price + l*l_price;
            if min_price > price { 
                min_price = price; 
                // println!("{} {} {}", s, m, l);
            }
        }
    }
    print!("{}", min_price);
}
