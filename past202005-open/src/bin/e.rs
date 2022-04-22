fn main() {
    proconio::input! {
        N: usize,
        M: usize,
        Q: usize,
        uv: [(usize, usize); M],
        mut c: [usize; N],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; N];
    uv.iter().for_each(|(u, v)| {
        let u_index = u - 1;
        let v_index = v - 1;

        g[u_index].push(v_index);
        g[v_index].push(u_index);
    });

    (0..Q).for_each(|_| {
        proconio::input! {
            t: usize,
            x: usize,
        }

        let x_index = x - 1;

        println!("{}", c[x_index]);

        match t {
            1 => {
                for i in &g[x_index] {
                    c[*i] = c[x_index];
                }
            }
            2 => {
                proconio::input! {
                    y: usize,
                }

                c[x_index] = y;
            }
            _ => panic!(),
        }
    });
}
