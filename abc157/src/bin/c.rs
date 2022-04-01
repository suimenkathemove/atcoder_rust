fn main() {
    proconio::input! {
        c: [[isize; 3]; 3],
    }

    let mut answer = true;

    let b1_b2_0 = c[0][0] - c[0][1];
    let b1_b2_1 = c[1][0] - c[1][1];
    let b1_b2_2 = c[2][0] - c[2][1];
    if b1_b2_0 != b1_b2_1 || b1_b2_1 != b1_b2_2 {
        answer = false;
    }

    let b2_b3_0 = c[0][1] - c[0][2];
    let b2_b3_1 = c[1][1] - c[1][2];
    let b2_b3_2 = c[2][1] - c[2][2];
    if b2_b3_0 != b2_b3_1 || b2_b3_1 != b2_b3_2 {
        answer = false;
    }

    let a1_a2_0 = c[0][0] - c[1][0];
    let a1_a2_1 = c[0][1] - c[1][1];
    let a1_a2_2 = c[0][2] - c[1][2];
    if a1_a2_0 != a1_a2_1 || a1_a2_1 != a1_a2_2 {
        answer = false;
    }

    let a2_a3_0 = c[1][0] - c[2][0];
    let a2_a3_1 = c[1][1] - c[2][1];
    let a2_a3_2 = c[1][2] - c[2][2];
    if a2_a3_0 != a2_a3_1 || a2_a3_1 != a2_a3_2 {
        answer = false;
    }

    println!("{}", if answer { "Yes" } else { "No" });
}
