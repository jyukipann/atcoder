use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[u8; w]; h]
    }
    let mut row: Vec<usize> = vec![0; w as usize];
    let mut columm: Vec<usize> = vec![0; h as usize];

    for i in 0..h {
        for j in 0..w {
            let aij = a[i][j] as usize;
            row[j] += aij;
            columm[i] += aij;
        }
    }

    let mut out:String = "".to_string();
    for i in 0..h {
        for j in 0..w {
            let aij = a[i][j] as usize;
            out += &((row[j] + columm[i] - aij).to_string() + " ");
        }
        out.pop();
        out += "\n";
    }
    out.pop();
    print!("{}", out);
}
