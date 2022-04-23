fn main() {
    proconio::input! {
        N: usize,
        Q: usize,
    }

    let mut g = vec![vec![false; N]; N];

    (0..Q).for_each(|_| {
        proconio::input! {
            t: usize,
            a: usize,
        }

        let a_index = a - 1;

        match t {
            1 => {
                proconio::input! {
                    b: usize,
                }

                let b_index = b - 1;

                g[a_index][b_index] = true;
            }
            2 => {
                (0..N).for_each(|i| {
                    if g[i][a_index] {
                        g[a_index][i] = true;
                    }
                });
            }
            3 => {
                let mut followings: Vec<usize> = vec![];

                (0..N).for_each(|i| {
                    if g[a_index][i] {
                        (0..N).for_each(|j| {
                            if g[i][j] && j != a_index {
                                followings.push(j);
                            }
                        });
                    }
                });

                followings.iter().for_each(|&i| {
                    g[a_index][i] = true;
                });
            }
            _ => (),
        }
    });

    (0..N).for_each(|i| {
        let s = (0..N).fold("".to_string(), |acc, j| {
            acc + if g[i][j] { "Y" } else { "N" }
        });
        println!("{}", s);
    });
}
