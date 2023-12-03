use proconio::input;

fn main() {
    input! {
        n : usize,
        a : [isize; n],
    }
    let mut a_with_i: Vec<(usize, &isize)> = a.iter().enumerate().collect();
    a_with_i.sort_by(|(_, av), (_, bv)| av.cmp(bv));
    let mut b = vec![0; n];
    for (i, ai) in a_with_i {
        a_with_i = a_with_i.iter().filter(|(_, av)| **av > ai).collect();
        b[i] = a_with_i.iter().sum();
    }
    print!(
        "{}",
        b.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
