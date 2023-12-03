use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n : usize,
        mut a : [isize; n],
    }
    
    let mut a_map: HashMap<isize, Vec<usize>> = HashMap::new();
    for (i, &value) in a.iter().enumerate() {
        a_map.entry(value).or_insert(vec![]).push(i);
    }

    let a: HashSet<_> = a.iter().cloned().collect();
    let mut a: Vec<_> = a.into_iter().collect();
    a.sort_by(|x, y| y.cmp(x));

    let mut b = vec![0_isize; n];
    let mut sum = 0;
    for ai in a {
        for &i in &a_map[&ai] {
            b[i] = sum;
        }
        sum += ai*(a_map[&ai].len() as isize);
    }
    print!(
        "{}",
        b.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
}
