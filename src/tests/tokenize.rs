use crate::parse::tokenize;
use crate::parse::Token::{AddSymbol, MulSymbol, Value, Variable};

#[test]
fn simple_tokenization() {
    assert_eq!(tokenize::<f64>("2+2".into()), Ok(vec![Value(2.0), AddSymbol, Value(2.0)]));
    assert_eq!(tokenize::<f64>("a".into()), Ok(vec![Variable("a".into())]));
    assert_eq!(tokenize::<f64>("ab".into()), Ok(vec![Variable("a".into()), MulSymbol, Variable("b".into())]));
    assert_eq!(tokenize::<f64>("\"abc\"\"bcd\"".into()), Ok(vec![Variable("abc".into()), MulSymbol, Variable("bcd".into())]));
    assert_eq!(tokenize::<f64>("\"ab\"+\"bc\"".into()), Ok(vec![Variable("ab".into()), AddSymbol, Variable("bc".into())]));
}
