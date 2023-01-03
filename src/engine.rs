mod codegen;
mod evaluator;
mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}

/// 正規表現と文字列をマッチングする
///
/// # 利用例
///
/// ```text
/// use regex;
/// regex::do_matching("abc|(de|cd)+", "decddede", true);
/// ```
///
/// # 引数
///
/// exprに正規表現。lineに対象の文字列を指定する
///
/// # 戻り値
///
/// マッチングに成功したらOk(true)
/// マッチングに失敗したらOk(false)
/// 入力の正規表現が不正な値であったり、内部的な実装エラー時はErrを返す
///
pub fn do_matching(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, is_depth)?)
}
/// 正規表現をパースしてコード生成し、
/// ASTと命令列を標準出力に表示する
pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST: {:?}", ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }

    Ok(())
}
