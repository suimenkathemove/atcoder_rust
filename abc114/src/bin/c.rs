fn main() {
    proconio::input! {
        n: usize,
    }

    let mut answer = 0;

    fn dfs(
        n: usize,
        answer: &mut usize,
        current: usize,
        new: usize,
        includes_3: bool,
        includes_5: bool,
        includes_7: bool,
    ) {
        let new_current: usize = (current.to_string() + &new.to_string()).parse().unwrap();

        if new_current > n {
            return;
        }

        if includes_3 && includes_5 && includes_7 {
            *answer += 1;
        }

        dfs(n, answer, new_current, 3, true, includes_5, includes_7);
        dfs(n, answer, new_current, 5, includes_3, true, includes_7);
        dfs(n, answer, new_current, 7, includes_3, includes_5, true);
    }
    dfs(n, &mut answer, 0, 3, true, false, false);
    dfs(n, &mut answer, 0, 5, false, true, false);
    dfs(n, &mut answer, 0, 7, false, false, true);

    println!("{}", answer);
}
