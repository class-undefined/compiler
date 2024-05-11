use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

use clap::{App, Arg};
use compiler::calculator1;
use compiler::calculator2;
use compiler::calculator3;
use compiler::calculator4;
use compiler::calculator5;
use compiler::sysy;
use lalrpop_util::lalrpop_mod;
mod ast;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut koopa = String::new();
    let mut output_path = String::new();

    let mut args_iter = args.iter();
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-koopa" => koopa = args_iter.next().cloned().unwrap_or_default(),
            "-o" => {
                output_path = args_iter.next().cloned().unwrap_or_default();
            }
            _ => {}
        }
    }
    let s = fs::read_to_string(koopa).unwrap();
    let unit = sysy::CompUnitParser::new().parse(s.as_str()).unwrap();
    let mut file = File::create(output_path).expect("create failed");
    write!(&mut file, "{}", unit.func_def.to_ir()).expect("write failed");
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

#[test]
fn calculator5() {
    let ast = calculator5::ExprParser::new().parse("(1 + (2 * 3))");
    let i = ast.unwrap();
    assert_eq!(format!("{:?}", i), "(1 + (2 * 3))")
}

#[test]
fn sysy1() {
    let input = r#"int main() {
        return 0;
      }
      "#;
    let unit = sysy::CompUnitParser::new().parse(input).unwrap();
    println!("{}", unit.func_def.to_ir());
}
