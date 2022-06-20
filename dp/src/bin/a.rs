fn main() {
    proconio::input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![std::isize::MAX; n];

    dp[0] = 0;

    (1..n).for_each(|i| {
        dp[i] = dp[i].min(dp[i - 1] + (h[i] - h[i - 1]).abs());

        if i >= 2 {
            dp[i] = dp[i].min(dp[i - 2] + (h[i] - h[i - 2]).abs());
        }
    });

    println!("{}", dp[n - 1]);
}
