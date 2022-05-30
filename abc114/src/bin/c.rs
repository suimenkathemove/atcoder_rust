use proconio::input;

fn main() {
    input! {
        N: usize,
    }

    let mut ans = 0;

    fn dfs(current: usize, N: usize, ans: &mut usize, has_3: bool, has_5: bool, has_7: bool) {
        if current > N {
            return;
        }

        if has_3 && has_5 && has_7 {
            *ans += 1;
        }

        dfs(current * 10 + 3, N, ans, true, has_5, has_7);
        dfs(current * 10 + 5, N, ans, has_3, true, has_7);
        dfs(current * 10 + 7, N, ans, has_3, has_5, true);
    }

    dfs(0, N, &mut ans, false, false, false);

    println!("{}", ans);
}
