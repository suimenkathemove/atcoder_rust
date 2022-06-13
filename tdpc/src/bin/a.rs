fn main() {
    proconio::input! {
        N: usize,
    }

    let ps = {
        let mut ps = vec![0];
        (0..N).for_each(|_| {
            proconio::input! {
                p: usize,
            }
            ps.push(p);
        });
        ps
    };

    let ps_sum = ps.iter().sum();

    let mut dp = vec![vec![false; ps_sum + 1]; N + 1];

    dp[0][0] = true;

    (1..=N).for_each(|n| {
        (0..=ps_sum).for_each(|p_sum| {
            if dp[n - 1][p_sum] || (p_sum >= ps[n] && dp[n - 1][p_sum - ps[n]]) {
                dp[n][p_sum] = true;
            }
        });
    });

    let ans = dp[N].iter().filter(|&b| *b).collect::<Vec<_>>().len();
    println!("{}", ans);
}
