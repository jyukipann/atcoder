use proconio::input;

fn main() {
    input! {
        n : usize,
        a : [isize; n],
    }
    let mut a_with_i: Vec<(usize, isize)> = a.iter()
        .enumerate()
        .map(|(i, av)| (i, av.clone()))
        .collect();
    a_with_i.sort_by(|(_, av), (_, bv)| av.cmp(bv));
    let mut a_with_i_filtered: Vec<(usize, isize)> = a_with_i.iter().cloned().collect();
    let mut b = vec![0; n];
    for (i, ai) in &a_with_i {
        a_with_i_filtered = a_with_i_filtered.iter().cloned().filter(|(_, av)| av > ai).collect();
        b[*i] = a_with_i_filtered.iter().map(|(_, av)| *av).sum();
    }
    print!(
        "{}",
        b.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
