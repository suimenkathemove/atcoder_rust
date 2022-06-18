fn main() {
    proconio::input! {
        n: usize,
    }

    let bs = {
        let mut bs = vec![0, 0];
        (2..=n).for_each(|_| {
            proconio::input! {
                b: usize,
            }
            bs.push(b);
        });
        bs
    };

    let mut s = vec![0; n + 1];

    (1..=n).rev().for_each(|i| {
        let children = bs
            .iter()
            .enumerate()
            .filter_map(|(index, &b)| {
                if b == i {
                    return Some(index);
                }
                None
            })
            .collect::<Vec<_>>();
        if children.is_empty() {
            s[i] = 1;
        } else {
            let cs = children.iter().map(|&c| s[c]).collect::<Vec<_>>();
            s[i] = cs.iter().max().unwrap() + cs.iter().min().unwrap() + 1;
        }
    });

    println!("{}", s[1]);
}
