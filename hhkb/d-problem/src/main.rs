use proconio::input;
use std::collections::HashSet;

fn dfs(graph: &Vec::<HashSet::<usize>>, x:&mut Vec::<isize>, node:usize, value: usize) -> bool{
    x[node] = value as isize;
    for i in &graph[node] {
        if (x[*i] == -1 && !dfs(graph, x, *i, 1 - value)) || x[node] == x[*i] {
            return false;
        }
    }
    return true;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        b: [usize; m],
    }

    let mut x = vec![-1_isize; n];
    let mut graph = vec![HashSet::<usize>::new(); n];
    for i in 0..m {
        graph[a[i]-1].insert(b[i]-1);
        graph[b[i]-1].insert(a[i]-1);
    }
    let graph = graph;

    for i in 0..n {
        if x[i] == -1 && !dfs(&graph, &mut x, i, 0){
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}
