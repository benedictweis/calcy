
fn main() {
    let result = calcy::solve("4+1+4*5*(5+6*(5-1))".into()).unwrap();
    println!("Result: {result}");
}
