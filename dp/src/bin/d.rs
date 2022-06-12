fn main() {
    proconio::input! {
        N: usize,
        W: usize,
    }

    let (ws, vs) = {
        let mut ws = Vec::<usize>::new();
        let mut vs = Vec::<usize>::new();
        (0..N).for_each(|_| {
            proconio::input! {
                (ww, vv): (usize, usize),
            }
            ws.push(ww);
            vs.push(vv);
        });
        (ws, vs)
    };

    let mut value = vec![vec![0; W]; N];

    value[0][ws[0] - 1] = vs[0];

    (1..N).for_each(|n| {
        (0..W).for_each(|w| {
            let v1 = value[n - 1][w];

            if w < ws[n] {
                value[n][w] = v1;

                return;
            }

            let v2 = value[n - 1][w - ws[n]] + vs[n];

            value[n][w] = std::cmp::max(v1, v2);
        });
    });

    println!("{}", value[N - 1].iter().max().unwrap());
}
