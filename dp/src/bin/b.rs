fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }

    let mut cost = vec![0; n];

    (1..n).for_each(|i| {
        cost[i] = (std::cmp::max(i as isize - k as isize, 0) as usize..i)
            .map(|j| cost[j] + (h[j] - h[i]).abs())
            .min()
            .unwrap();
    });

    println!("{}", cost[n - 1]);
}
