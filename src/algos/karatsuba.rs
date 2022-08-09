use core::cmp::max;

pub fn karatsuba(a: isize, b: isize) -> isize {
    // reference: https://codereview.stackexchange.com/questions/197974/karatsuba-multiplication-in-rust
    // base case for recursion, if a and b are single digits, return single digit multiplication
    if a < 10 && b < 10 {
        a * b
    } else {
        let half: u32 = max(get_half_digits(a), get_half_digits(b));

        let (p, q) = split_at_half(a, half);
        let (r, s) = split_at_half(b, half);

        let u = karatsuba(p, r);
        let v = karatsuba(p, s) + karatsuba(q, r);
        let w = karatsuba(q, s);


        // formula from: Stanford Course Algorithms - https://www.coursera.org/specializations/algorithms
        10_isize.pow(half * 2) * u +  10_isize.pow(half) * v + w
    }
}

fn get_half_digits(x: isize) -> u32 {
    let mut digits: u32 = 1;
    let mut x_copy: isize = x;

    while x_copy > 9 {
        x_copy = x_copy / 10;
        digits = digits + 1;
    }

    digits / 2
}

fn split_at_half(x: isize, half: u32) -> (isize, isize) {
    let x_copy: isize = x;
    let limit: isize = 10_isize.pow(half);

    let left_half: isize = x_copy / limit;
    let right_half: isize = x_copy %  limit;

    (left_half, right_half)
}

# [test]
fn test_get_no_of_digits() {
    assert_eq!(get_half_digits(123456), 3);
    assert_eq!(get_half_digits(1234), 2);
    assert_eq!(get_half_digits(12345), 2 );
}

# [test]
fn test_split_at_half() {
    assert_eq!(split_at_half(1234, 2), (12, 34));
    assert_eq!(split_at_half(123123, 3), (123, 123));
    assert_eq!(split_at_half(12345, 2), (12, 345));
}