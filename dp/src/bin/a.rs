fn main() {
    proconio::input! {
        n: usize,
        h: [isize; n],
    }

    let mut cost = vec![0; n];

    cost[1] = (h[0] - h[1]).abs();

    (2..n).for_each(|i| {
        cost[i] = std::cmp::min(
            cost[i - 2] + (h[i - 2] - h[i]).abs(),
            cost[i - 1] + (h[i - 1] - h[i]).abs(),
        );
    });

    println!("{}", cost[n - 1]);
}
