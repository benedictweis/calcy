use crate::parse::Expr::{Mul, Variable};
use crate::parse::{parse_string, tokenize, Expr, ParseError};

fn parse(input: String) -> Result<Expr<f64>, ParseError> {
    parse_string(tokenize::<f64>(input)?)
}

#[test]
fn simple_variable_parsing() {
    assert_eq!(parse("ab".into()), Ok(Mul(Box::new(Variable("a".into())), Box::new(Variable("b".into())))));
    assert_eq!(parse("\"a\"\"b\"".into()), Ok(Mul(Box::new(Variable("a".into())), Box::new(Variable("b".into())))));
    assert_eq!(parse("\"a\"b".into()), Ok(Mul(Box::new(Variable("a".into())), Box::new(Variable("b".into())))));
    assert_eq!(parse("a\"b\"".into()), Ok(Mul(Box::new(Variable("a".into())), Box::new(Variable("b".into())))));
    assert_eq!(parse("\"ab\"\"cd\"".into()), Ok(Mul(Box::new(Variable("ab".into())), Box::new(Variable("cd".into())))));
    assert_eq!(parse("\"abcdefg\"h".into()), Ok(Mul(Box::new(Variable("abcdefg".into())), Box::new(Variable("h".into())))));
}
