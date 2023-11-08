use proconio::input;

fn main() {
    input! {
        n:usize,
        cp:[[usize; 2]; n],
        q:usize,
        lr:[[usize; 2]; q]
    }

    let mut sum_p: Vec<Vec<usize>> = vec![vec![0, 0]; n + 1];
    for i in 0..n {
        sum_p[i+1][0] = sum_p[i][0];
        sum_p[i+1][1] = sum_p[i][1];
        sum_p[i+1][cp[i][0] - 1] += cp[i][1];
    }

    // sum_p.iter().for_each(|it| {
    //     println!("{:#?}", it);
    // });

    let mut out: String = "".to_string();
    for j in 0..q {
        let mut sum_ab: [usize; 2] = [0, 0];
        let lj = lr[j][0];
        let rj = lr[j][1];
        sum_ab[0] = sum_p[rj][0] - sum_p[lj-1][0];
        sum_ab[1] = sum_p[rj][1] - sum_p[lj-1][1];
        out += &format!("{} {}\n", sum_ab[0], sum_ab[1]);
    }
    out.pop();
    print!("{}", out);
}
