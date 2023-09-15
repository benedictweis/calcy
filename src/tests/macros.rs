use std::collections::HashMap;
use crate::solve;

#[test]
fn basic_solving() {
    solve!("2+2".into()).unwrap();
    solve!("2+a".into(), &HashMap::from([("a".to_string(), 1.0_f64)])).unwrap();
}