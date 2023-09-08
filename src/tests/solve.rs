use crate::{solve, Error};

fn assert_nearly_eq(result: Result<f64, Error>, expected: f64) {
    assert!((result.unwrap() - expected).abs() < 0.000000001);
}

#[test]
fn basic_addition() {
    assert_eq!(solve("5 + 3".into()), Ok(8.0));
    assert_eq!(solve("12 + 9".into()), Ok(21.0));
    assert_eq!(solve("18 + 6".into()), Ok(24.0));
    assert_eq!(solve("27 + 13".into()), Ok(40.0));
    assert_eq!(solve("4 + 11".into()), Ok(15.0));
    assert_eq!(solve("20 + 30".into()), Ok(50.0));
    assert_eq!(solve("8 + 16".into()), Ok(24.0));
    assert_eq!(solve("25 + 2".into()), Ok(27.0));
    assert_eq!(solve("7 + 14".into()), Ok(21.0));
    assert_eq!(solve("10 + 22".into()), Ok(32.0));
}

#[test]
fn basic_subtraction() {
    assert_eq!(solve("5 - 3".into()), Ok(2.0));
    assert_eq!(solve("12 - 9".into()), Ok(3.0));
    assert_eq!(solve("18 - 6".into()), Ok(12.0));
    assert_eq!(solve("27 - 13".into()), Ok(14.0));
    assert_eq!(solve("4 - 11".into()), Ok(-7.0));
    assert_eq!(solve("20 - 30".into()), Ok(-10.0));
    assert_eq!(solve("8 - 16".into()), Ok(-8.0));
    assert_eq!(solve("25 - 2".into()), Ok(23.0));
    assert_eq!(solve("7 - 14".into()), Ok(-7.0));
    assert_eq!(solve("10 - 22".into()), Ok(-12.0));
}

#[test]
fn basic_multiplication() {
    assert_eq!(solve("5 * 3".into()), Ok(15.0));
    assert_eq!(solve("12 * 9".into()), Ok(108.0));
    assert_eq!(solve("18 * 6".into()), Ok(108.0));
    assert_eq!(solve("27 * 13".into()), Ok(351.0));
    assert_eq!(solve("4 * 11".into()), Ok(44.0));
    assert_eq!(solve("20 * 30".into()), Ok(600.0));
    assert_eq!(solve("8 * 16".into()), Ok(128.0));
    assert_eq!(solve("25 * 2".into()), Ok(50.0));
    assert_eq!(solve("7 * 14".into()), Ok(98.0));
    assert_eq!(solve("10 * 22".into()), Ok(220.0));
}

#[test]
fn basic_division() {
    assert_eq!(solve("6 / 3".into()), Ok(2.0));
    assert_eq!(solve("27 / 9".into()), Ok(3.0));
    assert_eq!(solve("18 / 6".into()), Ok(3.0));
    assert_eq!(solve("39 / 13".into()), Ok(3.0));
    assert_eq!(solve("25 / 5".into()), Ok(5.0));
    assert_eq!(solve("50 / 10".into()), Ok(5.0));
    assert_eq!(solve("8 / 2".into()), Ok(4.0));
    assert_eq!(solve("100 / 25".into()), Ok(4.0));
    assert_eq!(solve("49 / 7".into()), Ok(7.0));
    assert_eq!(solve("99 / 11".into()), Ok(9.0));
}

#[test]
fn should_be_err() {
    assert!(solve("6 /".into()).is_err());
    assert!(solve("27*".into()).is_err());
    assert!(solve("18+++".into()).is_err());
    assert!(solve("39--12-343ü24ü234".into()).is_err());
    assert!(solve("25&+1".into()).is_err());
    assert!(solve("50--2".into()).is_err());
    assert!(solve("8++ß".into()).is_err());
    assert!(solve("afk#sdmf".into()).is_err());
    assert!(solve("".into()).is_err());
}

#[test]
fn edge_cases() {
    assert_eq!(solve("0*1*2*3*4*5*6*7".into()), Ok(0.0));
    assert_eq!(solve("0.5-0.5".into()), Ok(0.0));
    assert_eq!(solve("100-100".into()), Ok(0.0));
    assert_eq!(solve("0.5*0.5".into()), Ok(0.25));
    assert_nearly_eq(solve("0.1+0.2".into()), 0.3);
}
