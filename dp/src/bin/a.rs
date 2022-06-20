fn main() {
    proconio::input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = vec![std::isize::MAX; n];

    dp[0] = 0;

    dp[1] = (h[1] - h[0]).abs();

    (2..n).for_each(|i| {
        dp[i] = std::cmp::min(
            dp[i - 1] + (h[i] - h[i - 1]).abs(),
            dp[i - 2] + (h[i] - h[i - 2]).abs(),
        );
    });

    println!("{}", dp[n - 1]);
}
