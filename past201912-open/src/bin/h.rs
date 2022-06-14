fn main() {
    proconio::input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
    }

    let mut min_set = c
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            if i % 2 == 0 {
                return Some(x);
            }
            None
        })
        .min()
        .unwrap();
    let mut min_all = *c.iter().min().unwrap();

    let mut set_sell = 0;
    let mut all_sell = 0;

    let mut ans = 0;

    (0..q).for_each(|_| {
        proconio::input! {
            t: usize,
        }

        match t {
            1 => {
                proconio::input! {
                    x: usize,
                    a: usize,
                }

                let x = x - 1;

                if x % 2 == 0 {
                    if c[x] >= a + all_sell + set_sell {
                        c[x] -= a;

                        min_set = min_set.min(c[x] - all_sell - set_sell);
                        min_all = min_all.min(min_set);

                        ans += a;
                    }
                } else {
                    if c[x] >= a + all_sell {
                        c[x] -= a;

                        min_all = min_all.min(c[x] - all_sell);

                        ans += a;
                    }
                }
            }
            2 => {
                proconio::input! {
                    a: usize,
                }

                if min_set >= a {
                    min_set -= a;
                    min_all = min_all.min(min_set);

                    set_sell += a;

                    ans += a * ((c.len() as f64 / 2.).ceil() as usize);
                }
            }
            3 => {
                proconio::input! {
                    a: usize,
                }

                if min_all >= a {
                    min_all -= a;
                    min_set -= a;

                    all_sell += a;

                    ans += a * c.len();
                }
            }
            _ => panic!(),
        }
    });

    println!("{}", ans);
}
