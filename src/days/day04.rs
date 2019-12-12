#![allow(dead_code)]

fn digits_iter(num: usize) -> impl Iterator<Item = u8> + 'static {
    let mut num = num;
    [100_000, 10_000, 1_000, 100, 10, 1]
        .iter()
        .map(move |base| {
            let digit = (num / base) as u8;
            num %= base;
            digit
        })
}

fn is_valid_pw(pw: usize) -> bool {
    if pw < 100_000 || pw > 1_000_000 {
        return false;
    }

    let mut last_digit = 0;
    let mut same = false;

    for digit in digits_iter(pw) {
        if last_digit > digit {
            return false;
        }

        if last_digit == digit {
            same = true;
        }

        last_digit = digit;
    }

    same
}

fn is_valid_pw2(pw: usize) -> bool {
    if pw < 100_000 || pw > 1_000_000 {
        return false;
    }

    let mut last_digit = 0;
    let mut same = false;
    let mut same_counter = 1;
    for digit in digits_iter(pw) {
        if last_digit > digit {
            return false;
        }

        if last_digit == digit {
            same_counter += 1;
        } else {
            if same_counter == 2 {
                same = true;
            }

            same_counter = 1;
        }

        last_digit = digit;
    }

    if same_counter == 2 {
        same = true;
    }

    same
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        assert!(is_valid_pw(111_111));
        assert!(!is_valid_pw(223_450));
        assert!(!is_valid_pw(123_789));

        let n = (234_208..765_869).filter(|&pw| is_valid_pw(pw)).count();

        assert_eq!(n, 1246);
        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        assert!(is_valid_pw2(112_233));
        assert!(!is_valid_pw2(123_444));
        assert!(is_valid_pw2(111_122));

        let n = (234_208..765_869).filter(|&pw| is_valid_pw2(pw)).count();

        assert_eq!(n, 814);

        Ok(())
    }
}
