fn dup_digits(pwd: u32) -> bool {
    let mut shifted = pwd / 10;
    let mut original = pwd;

    while shifted > 0 && original > 0 {
        if shifted % 10 == original % 10 {
            return true;
        }

        shifted /= 10;
        original /= 10;
    }

    false
}

fn non_descending(pwd: u32) -> bool {
    let mut shifted = pwd / 10;
    let mut original = pwd;

    while shifted > 0 && original > 0 {
        if shifted % 10 > original % 10 {
            return false;
        }

        shifted /= 10;
        original /= 10;
    }

    true
}

fn main() {
    let mut count = 0;

    for n in 146810..=612564 {
        if dup_digits(n) && non_descending(n) {
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
    fn detects_non_descending() {
        assert!(!non_descending(543210));
        assert!(non_descending(555555));
        assert!(non_descending(555556));
        assert!(!non_descending(555565));
    }
}
