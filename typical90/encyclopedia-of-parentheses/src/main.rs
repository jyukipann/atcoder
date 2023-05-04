use proconio::input;

fn parentheses(n: u8) -> Vec<u32> {
    if n == 2 {
        return vec![0b01 as u32];
    }
    let mut result: Vec<u32> = Vec::new();
    if (n - 2) % 2 == 0 {
        let _n: u8 = n - 2;
        for bit in parentheses(_n).into_iter() {
            let bit: u32 = bit << 1;
            let bit: u32 = (1 << (_n + 1)) | bit;
            result.push(bit);
        }
    }
    
    return vec![0 as u32];
}

fn main() {
    input! {
        n:u8,
    }
    if n % 2 == 1 {
        println!("");
    }
}
