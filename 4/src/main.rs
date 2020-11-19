use itertools::Itertools;

fn least_significant_digits(number: u32) -> Vec<u8> {
    let mut ds = vec![];
    let mut remaining = number;

    while remaining > 0 {
        ds.push((remaining % 10) as u8);
        remaining /= 10;
    }

    ds
}

fn dup_digits(pwd: u32) -> bool {
    let shifted = &least_significant_digits(pwd)[1..];
    let original = least_significant_digits(pwd);

    for (next, current) in shifted.iter().zip(original) {
        if *next == current {
            return true;
        }
    }

    false
}

fn dup_digits_but_not_trip(pwd: u32) -> bool {
    let shifted = &least_significant_digits(pwd)[1..];
    let original = least_significant_digits(pwd);

    return shifted
        .iter()
        .zip(original)
        .group_by(|(next, current)| *next == current)
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, g)| g.count())
        .find(|group_count| *group_count == 1)
        .map(|_| true)
        .unwrap_or(false);
}

fn non_descending(pwd: u32) -> bool {
    let shifted = &least_significant_digits(pwd)[1..];
    let original = least_significant_digits(pwd);

    for (next, current) in shifted.iter().zip(original) {
        if *next > current {
            return false;
        }
    }

    true
}

fn main() {
    let mut count = 0;

    for n in 146810..=612564 {
        if dup_digits_but_not_trip(n) && non_descending(n) {
            count += 1;
        }
    }

    println!("Number of candidates: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_duplicates() {
        assert!(!dup_digits(123456));
        assert!(dup_digits(123455));
        assert!(dup_digits(113456));
    }

    #[test]
    fn detects_dup_but_not_trip() {
        assert!(dup_digits_but_not_trip(123455));
        assert!(!dup_digits_but_not_trip(123555));
        assert!(dup_digits_but_not_trip(112555));
    }

    #[test]
    fn detects_non_descending() {
        assert!(!non_descending(543210));
        assert!(non_descending(555555));
        assert!(non_descending(555556));
        assert!(!non_descending(555565));
    }
}
