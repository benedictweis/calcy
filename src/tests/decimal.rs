use crate::decimal::Decimal;
use crate::solve_with;
use std::str::FromStr;

#[test]
fn parse() {
    assert_eq!(Decimal::from_str("0"), Ok(Decimal::new(0, 0)));
    assert_eq!(Decimal::from_str("1"), Ok(Decimal::new(1, 0)));
    assert_eq!(Decimal::from_str("1.1"), Ok(Decimal::new(11, 1)));
    assert_eq!(Decimal::from_str("11.11"), Ok(Decimal::new(1111, 2)));
    assert_eq!(Decimal::from_str("11.111"), Ok(Decimal::new(11111, 3)));
    assert_eq!(Decimal::from_str("1.2345"), Ok(Decimal::new(12345, 4)));
}

#[test]
fn display() {
    assert_eq!(Decimal::from_str("0").unwrap().to_string(), "0".to_string());
    assert_eq!(Decimal::from_str("1").unwrap().to_string(), "1".to_string());
    assert_eq!(Decimal::from_str("1.1").unwrap().to_string(), "1.1".to_string());
    assert_eq!(Decimal::from_str("11.11").unwrap().to_string(), "11.11".to_string());
    assert_eq!(Decimal::from_str("11.111").unwrap().to_string(), "11.111".to_string());
    assert_eq!(Decimal::from_str("1.2345").unwrap().to_string(), "1.2345".to_string());
}

#[test]
fn scale() {
    let mut a = Decimal::new(0, 0);
    a.scale(1);
    let b = Decimal::new(0, 1);
    assert_eq!(a, b);
    let mut a = Decimal::new(1, 0);
    a.scale(1);
    let b = Decimal::new(10, 1);
    assert_eq!(a, b);
    let mut a = Decimal::new(12345, 3);
    a.scale(5);
    let b = Decimal::new(1234500, 5);
    assert_eq!(a, b);
}

#[test]
fn add() {
    assert_eq!(solve_with("0+0".into()), Ok(Decimal::from_str("0").unwrap()));
    assert_eq!(solve_with("1+2".into()), Ok(Decimal::from_str("3").unwrap()));
    assert_eq!(solve_with("0.1+0.2".into()), Ok(Decimal::from_str("0.3").unwrap()));
}

#[test]
fn sub() {
    //assert_eq!(solve_with("0-0".into()), Ok(Decimal::from_str("0").unwrap()));
    //assert_eq!(solve_with("1-2".into()), Ok(Decimal::from_str("-1").unwrap()));
    //assert_eq!(solve_with("0.1-0.2".into()), Ok(Decimal::from_str("-0.1").unwrap()));
    //assert_eq!(solve_with("0.1-0.1".into()), Ok(Decimal::from_str("0").unwrap()));
    assert_eq!(solve_with("0.25-0.35".into()), Ok(Decimal::from_str("-0.1").unwrap()));
}
