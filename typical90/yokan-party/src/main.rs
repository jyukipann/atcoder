use proconio::input;

// target_lengthより長いピースで分割できるか
// 分割できないとき、切ったあとのピースはk+1より少なくなる。そのためtarget_lengthをこれ以上長くできない。
fn can_cut(target_length: usize, k: usize, subpieces: &Vec<usize>) -> (bool, usize) {
    let mut current_piece: usize = 1;
    let mut piece_count: usize = 0;
    let mut score: usize = std::usize::MAX;
    for subpiece in subpieces.into_iter() {
        // println!(
        //     "can cut {} {} {} {}",
        //     score, current_piece, subpiece, piece_count
        // );
        current_piece += subpiece;
        if current_piece >= target_length {
            piece_count += 1;
            if score > current_piece {
                score = current_piece;
            }
            current_piece = 0;
        }
        
    }

    return (piece_count >= k+1, score);
}

/*
 * 二分探索で頑張る必要あり
 * 最小値の最大化の問題である。
 */

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n]
    }

    a.insert(0, 0);
    a.push(l);
    let a = a;

    let mut subpieces: Vec<usize> = Vec::with_capacity(n + 1);
    for i in 1..n + 2 {
        subpieces.push(a[i] - a[i - 1]);
    }
    let subpieces = subpieces;
    // println!("{:#?}", subpieces);

    let mut left: usize = 0;
    let mut right: usize = l;
    let mut mid = (left + right) / 2;
    let mut score: usize = std::usize::MAX;

    while right - left > 1 {
        let (longer, current_score) = can_cut(mid, k, &subpieces);
        // println!("main {} {} {} {} {} {}", left, mid, right, score, current_score, longer );
        if longer {
            left = mid+1;
            score = current_score;
        } else {
            right = mid;
        }
        mid = (left + right) / 2;
    }

    println!("{}", score);
}
