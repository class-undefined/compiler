use compiler::calculator1;
use compiler::calculator2;
use compiler::calculator3;
use compiler::calculator4;
use lalrpop_util::lalrpop_mod;

mod ast;
fn main() {
    // lalrpop_mod!(pub calculator1);
}
#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator2::TermParser::new().parse("((22)").is_err());
}

#[test]
fn calculator3() {
    let ast = calculator3::ExprParser::new().parse("(1 + (2 * 3))");
    let i = ast.unwrap();
    assert_eq!(i, 7);
}

#[test]
fn calculator4() {
    let ast = calculator4::ExprParser::new().parse("(1 + (2 * 3))");
    let i = ast.unwrap();
    assert_eq!(i, 7);
}
