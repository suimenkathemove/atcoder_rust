fn main() {
    proconio::input! {
        N: usize,
        M: usize,
        Q: usize,
        uv: [(usize, usize); M],
        mut c: [usize; N],
    }

    let g = {
        let mut g = vec![vec![]; N];

        uv.iter().for_each(|(u, v)| {
            let u_index = u - 1;
            let v_index = v - 1;

            g[u_index].push(v_index);
            g[v_index].push(u_index);
        });

        g
    };

    (0..Q).for_each(|_| {
        proconio::input! {
            t: usize,
            x: usize,
        }

        let x_index = x - 1;

        println!("{}", c[x_index]);

        match t {
            1 => {
                g[x_index].iter().for_each(|i| {
                    c[*i] = c[x_index];
                });
            }
            2 => {
                proconio::input! {
                    y: usize,
                }

                c[x_index] = y;
            }
            _ => (),
        }
    });
}
