fn main() {
    proconio::input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }

    let mut bingo: [[bool; 3]; 3] = [[false; 3]; 3];

    b.iter().for_each(|&bb| {
        (0..3).for_each(|i| {
            (0..3).for_each(|j| {
                if a[i][j] == bb {
                    bingo[i][j] = true;
                }
            })
        });
    });

    let mut answer = false;

    (0..3).for_each(|i| {
        if bingo[i][0] && bingo[i][1] && bingo[i][2] {
            answer = true;
        }

        if bingo[0][i] && bingo[1][i] && bingo[2][i] {
            answer = true;
        }
    });

    if bingo[0][0] && bingo[1][1] && bingo[2][2] {
        answer = true;
    }

    if bingo[0][2] && bingo[1][1] && bingo[2][0] {
        answer = true;
    }

    println!("{}", if answer { "Yes" } else { "No" });
}
