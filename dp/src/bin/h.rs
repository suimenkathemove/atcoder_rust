fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        x: [usize; n],
        t1: usize,
        t2: usize,
        t3: usize,
    }

    let h = {
        let mut h = vec![false; l + 1];
        x.iter().for_each(|&i| {
            h[i] = true;
        });
        h
    };

    let mut t = vec![std::usize::MAX; l + 1];

    t[0] = 0;

    (1..=l).for_each(|i| {
        t[i] = std::cmp::min(t[i], t[i - 1] + t1);

        if i >= 2 {
            t[i] = std::cmp::min(t[i], t[i - 2] + t1 + t2);
        }

        if i >= 4 {
            t[i] = std::cmp::min(t[i], t[i - 4] + t1 + 3 * t2);
        }

        if h[i] {
            t[i] += t3;
        }
    });

    (1..=3).for_each(|i| {
        if l < i {
            return;
        }

        t[l] = std::cmp::min(t[l], t[l - i] + t1 / 2 + (i - 1) * t2 + t2 / 2);
    });

    println!("{}", t[l]);
}
