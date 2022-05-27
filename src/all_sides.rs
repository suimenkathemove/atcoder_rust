pub fn all_sides2((i, j): (usize, usize)) -> Vec<(usize, usize)> {
    let i = i as isize;
    let j = j as isize;

    [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
        .iter()
        .filter_map(|&(i, j)| {
            let option_i = usize::try_from(i).ok();
            let option_j = usize::try_from(j).ok();

            if option_i == None || option_j == None {
                return None;
            }

            Some((option_i.unwrap(), option_j.unwrap()))
        })
        .collect()
}

pub fn all_sides((i, j): (usize, usize)) -> Vec<(usize, usize)> {
    let mut all_sides = vec![];

    if i > 0 {
        all_sides.push((i - 1, j));
    }

    all_sides.push((i + 1, j));

    if j > 0 {
        all_sides.push((i, j - 1));
    }

    all_sides.push((i, j + 1));

    all_sides
}

#[cfg(test)]
mod tests {
    use crate::all_sides::all_sides;

    #[test]
    fn one_one() {
        assert_eq!(all_sides((1, 1)), vec![(0, 1), (2, 1), (1, 0), (1, 2)]);
    }

    #[test]
    fn one_zero() {
        assert_eq!(all_sides((1, 0)), vec![(0, 0), (2, 0), (1, 1)]);
    }

    #[test]
    fn zero_one() {
        assert_eq!(all_sides((0, 1)), vec![(1, 1), (0, 0), (0, 2)]);
    }

    #[test]
    fn zero_zero() {
        assert_eq!(all_sides((0, 0)), vec![(1, 0), (0, 1)]);
    }
}
