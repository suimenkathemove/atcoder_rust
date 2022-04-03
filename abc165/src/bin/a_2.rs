fn main() {
    proconio::input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let mut answer = false;

    let mut count = 1;

    while k * count <= b {
        let n = k * count;
        if a <= n && n <= b {
            answer = true;

            break;
        }

        count += 1;
    }

    println!("{}", if answer { "OK" } else { "NG" });
}
