fn main() {
    proconio::input! {
        N: usize,
        W: usize,
    }

    let (ws, vs) = {
        let mut ws = vec![0];
        let mut vs = vec![0];
        (0..N).for_each(|_| {
            proconio::input! {
                (ww, vv): (usize, usize),
            }
            ws.push(ww);
            vs.push(vv);
        });
        (ws, vs)
    };

    let vs_sum = vs.iter().sum();

    let mut cost = vec![vec![10usize.pow(9) + 1; vs_sum + 1]; N + 1];

    cost[0][0] = 0;

    (1..=N).for_each(|n| {
        (0..=vs_sum).for_each(|v| {
            cost[n][v] = cost[n - 1][v];

            if v >= vs[n] {
                cost[n][v] = std::cmp::min(cost[n][v], cost[n - 1][v - vs[n]] + ws[n]);
            }
        });
    });

    let ans = (0..=vs_sum)
        .filter_map(|v| {
            if cost[N][v] > W {
                return None;
            }
            Some(v)
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
