pub fn main() {
    let mut s1 = String::new();
    std::io::stdin().read_line(&mut s1).ok();

    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s2).ok();

    let mut dp = vec![vec![std::usize::MAX; s2.len() + 1]; s1.len() + 1];

    dp[0][0] = 0;

    (0..=s1.len()).for_each(|i| {
        (0..=s2.len()).for_each(|j| {
            // 編集
            if i >= 1 && j >= 1 {
                if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 1);
                }
            }

            // 削除
            if i >= 1 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }

            // 挿入
            if j >= 1 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }
        });
    });

    println!("{}", dp[s1.len()][s2.len()]);
}
