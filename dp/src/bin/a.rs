fn main() {
    proconio::input! {
        n: usize,
        h: [usize; n],
    }

    let mut cost = vec![0; n];

    cost[1] = (h[0] as isize - h[1] as isize).abs() as usize;

    (2..n).for_each(|i| {
        cost[i] = std::cmp::min(
            cost[i - 1] + (h[i - 1] as isize - h[i] as isize).abs() as usize,
            cost[i - 2] + (h[i - 2] as isize - h[i] as isize).abs() as usize,
        );
    });

    println!("{}", cost[n - 1]);
}
