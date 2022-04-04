use std::collections::HashSet;

fn main() {
    proconio::input! {
        S: String,
    }

    let mut set: HashSet<String> = HashSet::new();

    for start in 0..=S.len() - 1 {
        for gap in 0..=2 {
            let end = start + gap;

            if end > S.len() - 1 {
                break;
            }

            let base_str = &S[start..=end];

            fn combine_with_colon(
                current_str: &str,
                base_str: &str,
                index: usize,
                set: &mut HashSet<String>,
            ) {
                if index > base_str.len() - 1 {
                    return;
                }

                let normal_string =
                    format!("{}{}", current_str, base_str.chars().nth(index).unwrap());
                set.insert(normal_string.clone());
                combine_with_colon(&normal_string, base_str, index + 1, set);

                let colon_string = format!("{}{}", current_str, '.');
                set.insert(colon_string.clone());
                combine_with_colon(&colon_string, base_str, index + 1, set);
            }
            combine_with_colon("", base_str, 0, &mut set);
        }
    }

    println!("{}", set.len());
}
