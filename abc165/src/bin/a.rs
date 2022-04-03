fn main() {
    proconio::input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let mut answer = false;

    for i in a..=b {
        if i % k == 0 {
            answer = true;

            break;
        }
    }

    println!("{}", if answer { "OK" } else { "NG" });
}
