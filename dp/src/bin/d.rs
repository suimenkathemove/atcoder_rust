fn main() {
    proconio::input! {
        N: usize,
        W: usize,
        wv: [(usize, usize); N],
    }

    let mut dp = vec![vec![0; W + 1]; N];

    dp[0][wv[0].0] = wv[0].1;

    (1..N).for_each(|n| {
        (0..=W).for_each(|w| {
            dp[n][w] = dp[n][w].max(dp[n - 1][w]);

            if w >= wv[n].0 {
                dp[n][w] = dp[n][w].max(dp[n - 1][w - wv[n].0] + wv[n].1);
            }
        });
    });

    println!("{}", dp[N - 1].iter().max().unwrap());
}
