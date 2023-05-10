#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_hex() {
        let mut big_int = BigNumber::new();
        big_int.set_hex("12345");

        assert_eq!(big_int.data, vec![74565]);
    }

    #[test]
    fn test_get_hex() {
        let mut big_int = BigNumber::new();
        big_int.set_hex("12345");

        assert_eq!(big_int.get_hex(), String::from("12345"));
    }

    #[test]
    fn test_inv() {
        let mut a = MyBigInt::new();
        a.digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut b = MyBigInt::new();
        b.digits = vec![!0, !1, !2, !3, !4, !5, !6, !7, !8, !9];

        a.inv();
        assert_eq!(a.digits, b.digits);
    }

    #[test]
    fn test_xor() {
        let a = MyBigInt {
            digits: vec![0b1010, 0b0101, 0b1111, 0b0000],
            ..Default::default()
        };
        let b = MyBigInt {
            digits: vec![0b1100, 0b0011, 0b1010, 0b0101],
            ..Default::default()
        };
        let expected_result = MyBigInt {
            digits: vec![0b0110, 0b0110, 0b0101, 0b0101],
            ..Default::default()
        };
        let result = a.xor(&b);
        let left_trimmed = &result.digits[1..];
        let expected_trimmed = &expected_result.digits;
        assert_eq!(left_trimmed, expected_trimmed);
    }


    #[test]
    fn test_or() {
        let a = MyBigInt {
            digits: vec![0b1010, 0b0101, 0b1111, 0b0000],
            ..Default::default()
        };
        let b = MyBigInt {
            digits: vec![0b1100, 0b0011, 0b1010, 0b0101],
            ..Default::default()
        };
        let expected_result = MyBigInt {
            digits: vec![0b1110, 0b0111, 0b1111, 0b0101],
            ..Default::default()
        };
        let result = a.or(&b);
        let left_trimmed = &result.digits[1..];
        let expected_trimmed = &expected_result.digits;
        assert_eq!(left_trimmed, expected_trimmed);
    }

    #[test]
    fn test_and() {
        let mut big_int_1 = MyBigInt::new();
        big_int_1.set_hex("7");
        let mut big_int_2 = MyBigInt::new();
        big_int_2.set_hex("8");

        let result = big_int_1.and(&big_int_2);

        assert_eq!(result.digits, vec![0]);
    }

    #[test]
    fn test_shift_r() {
        let a = MyBigInt {
            digits: vec![0b1010, 0b0101, 0b1111, 0b0000],
            ..Default::default()
        };
        let expected_result = MyBigInt {
            digits: vec![0b0000, 0b1010, 0b0101, 0b1111],
            ..Default::default()
        };
        let result = a.shift_r(4);
        let left_trimmed = &result.digits[1..];
        let expected_trimmed = &expected_result.digits;
        assert_eq!(left_trimmed, expected_trimmed);

        let b = MyBigInt {
            digits: vec![0b0000, 0b1010, 0b0101, 0b1111],
            ..Default::default()
        };
        let expected_result = MyBigInt {
            digits: vec![ 0b0000, 0b1010, 0b0101, 0b1111],
            ..Default::default()
        };
        let result = b.shift_r(16);
        let left_trimmed = &result.digits[1..];
        let expected_trimmed = &expected_result.digits;
        assert_eq!(left_trimmed, expected_trimmed);
    }

    #[test]
    fn test_shift_l() {
        let a = MyBigInt {
            digits: vec![0b1010, 0b0101, 0b1111],
            ..Default::default()
        };
        let expected_result = MyBigInt {
            digits: vec![0b0101, 0b1111, 0b0000],
            ..Default::default()
        };
        let result = a.shift_l(4);
        let left_trimmed = &result.digits;
        let expected_trimmed = &expected_result.digits;
        assert_eq!(left_trimmed, expected_trimmed);

        let b = MyBigInt {
            digits: vec![0b1010, 0b0101, 0b1111],
            ..Default::default()
        };
        let expected_result = MyBigInt {
            digits: vec![0b1010, 0b0101, 0b1111],
            ..Default::default()
        };
        let result = b.shift_l(16);
        let left_trimmed = &result.digits;
        let expected_trimmed = &expected_result.digits;
        assert_eq!(left_trimmed, expected_trimmed);
    }

    #[test]
    fn test_add_positive() {
        let mut a = MyBigInt::new();
        a.set_hex("64");
        let mut b = MyBigInt::new();
        b.set_hex("C8");
        let mut expected = MyBigInt::new();
        expected.set_hex("12C");
        assert_eq!(a.add(&b), expected);
    }

    #[test]
    fn test_add_negative() {
        let mut a = MyBigInt::new();
        a.set_hex("-64");
        let mut b = MyBigInt::new();
        b.set_hex("-C8");
        let mut expected = MyBigInt::new();
        expected.set_hex("-12C");
        assert_eq!(a.add(&b), expected);
    }

    #[test]
    fn test_add_mixed_signs() {
        let mut a = MyBigInt::new();
        a.set_hex("64");
        let mut b = MyBigInt::new();
        b.set_hex("-C8");
        let mut expected = MyBigInt::new();
        expected.set_hex("-64");
        assert_eq!(a.add(&b), expected);
    }

    #[test]
    fn test_sub_positive() {
        let mut a = MyBigInt::new();
        a.set_hex("12C");
        let mut b = MyBigInt::new();
        b.set_hex("C8");
        let mut expected = MyBigInt::new();
        expected.set_hex("64");
        assert_eq!(a.sub(&b), expected);
    }

    #[test]
    fn test_sub_negative() {
        let mut a = MyBigInt::new();
        a.set_hex("-12C");
        let mut b = MyBigInt::new();
        b.set_hex("-C8");
        let mut expected = MyBigInt::new();
        expected.set_hex("-64");
        assert_eq!(a.sub(&b), expected);
    }

    #[test]
    fn test_sub_mixed_signs() {
        let mut a = MyBigInt::new();
        a.set_hex("64");
        let mut b = MyBigInt::new();
        b.set_hex("-C8");
        let mut expected = MyBigInt::new();
        expected.set_hex("12C");
        assert_eq!(a.sub(&b), expected);
    }

    #[test]
    fn test_modulus_positive() {
        let mut a = MyBigInt::new();
        a.set_hex("7");
        let mut b = MyBigInt::new();
        b.set_hex("3");
        let mut expected = MyBigInt::new();
        expected.set_hex("1");
        assert_eq!(a.modulus(&b), expected);
    }

    #[test]
    fn test_modulus_negative() {
        let mut a = MyBigInt::new();
        a.set_hex("-7");
        let mut b = MyBigInt::new();
        b.set_hex("3");
        let mut expected = MyBigInt::new();
        expected.set_hex("2");
        assert_eq!(a.modulus(&b), expected);
    }

    #[test]
    fn test_modulus_mixed_signs() {
        let mut a = MyBigInt::new();
        a.set_hex("-7");
        let mut b = MyBigInt::new();
        b.set_hex("-3");
        let mut expected = MyBigInt::new();
        expected.set_hex("2");
        assert_eq!(a.modulus(&b), expected);
    }
}
