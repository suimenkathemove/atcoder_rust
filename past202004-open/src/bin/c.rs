fn main() {
    proconio::input! {
        n: usize,
        s_rows: [String; n],
    }

    let mut s: Vec<Vec<char>> = s_rows.iter().map(|s_row| s_row.chars().collect()).collect();

    (0..n - 1).rev().for_each(|i| {
        (1..(2 * n - 1) - 1).for_each(|j| {
            if s[i][j] == '#' && (j - 1..=j + 1).any(|n| s[i + 1][n] == 'X') {
                s[i][j] = 'X';
            }
        });
    });

    (0..n).for_each(|i| {
        println!("{}", s[i].iter().collect::<String>());
    });
}
