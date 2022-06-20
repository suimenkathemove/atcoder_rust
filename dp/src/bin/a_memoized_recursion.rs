fn main() {
    proconio::input! {
        n: usize,
        h: [isize; n],
    }

    let initial_value = -1;
    let mut dp = vec![initial_value; n];

    fn rec(i: usize, dp: &mut Vec<isize>, initial_value: isize, h: &[isize]) -> isize {
        if dp[i] != initial_value {
            return dp[i];
        }

        if i == 0 {
            return 0;
        }

        let mut cost = rec(i - 1, dp, initial_value, h) + (h[i] - h[i - 1]).abs();
        if i >= 2 {
            cost = cost.min(rec(i - 2, dp, initial_value, h) + (h[i] - h[i - 2]).abs());
        }

        dp[i] = cost;

        cost
    }
    println!("{}", rec(n - 1, &mut dp, initial_value, &h));
}
