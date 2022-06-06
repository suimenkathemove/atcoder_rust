fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }

    let mut cost = vec![0; n];

    cost[1] = (h[0] as isize - h[1] as isize).abs() as usize;

    (2..n).for_each(|i| {
        cost[i] = (std::cmp::max(i as isize - k as isize, 0) as usize..i)
            .map(|j| cost[j] + (h[i] as isize - h[j] as isize).abs() as usize)
            .min()
            .unwrap();
    });

    println!("{}", cost[n - 1]);
}
