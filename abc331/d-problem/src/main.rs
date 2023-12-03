use proconio::input;

fn main() {
    input! {
        n : usize,
        q : usize,
        board : [[String; n]; n],
        queries : [[usize; 4]; q],
    }
    let mut board_num = vec![vec![0_usize; n as usize]; n as usize];
    for i in 0..n {
        for j in 0..n {
            board_num[i][j] = if &*board[i][j] == "B" { 1 } else { 0 };

        }
    }
    let board = board_num;

    println!("{}", 0);
}
