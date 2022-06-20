fn main() {
    proconio::input! {
        n: usize,
        h: [isize; n],
    }

    let mut cost = vec![std::isize::MAX; n];

    cost[0] = 0;

    cost[1] = (h[1] - h[0]).abs();

    (2..n).for_each(|i| {
        cost[i] = std::cmp::min(
            cost[i - 1] + (h[i] - h[i - 1]).abs(),
            cost[i - 2] + (h[i] - h[i - 2]).abs(),
        );
    });

    println!("{}", cost[n - 1]);
}
