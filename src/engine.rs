mod codegen;
mod evaluator;
mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Dot,
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Dot => write!(f, "any character is ok"),
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
pub fn do_matching(
    expr: &str,
    line: &str,
    is_depth: bool,
) -> Result<(bool, Option<String>), DynError> {
    let ast_state = parser::parse(expr)?;
    let code = codegen::get_code(&ast_state.ast)?;

    let line = line.chars().collect::<Vec<char>>();

    if ast_state.has_hat {
        if evaluator::eval(&code, &line, is_depth)? {
            return Ok((true, Some(line.into_iter().collect::<String>())));
        } else {
            return Ok((false, None));
        }
    }

    if ast_state.has_doller {
        for i in (0..line.len()).rev() {
            let line = &line[i..];
            if evaluator::eval(&code, line, is_depth)? {
                return Ok((true, Some(line.into_iter().collect::<String>())));
            }
        }
    }

    for i in 0..line.len() {
        let line = &line[i..];
        if evaluator::eval(&code, line, is_depth)? {
            return Ok((true, Some(line.into_iter().collect::<String>())));
        }
    }
    Ok((false, None))
}
/// 正規表現をパースしてコード生成し、
/// ASTと命令列を標準出力に表示する
pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast_state = parser::parse(expr)?;
    println!("AST: {:?}", ast_state.ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast_state.ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }

    Ok(())
}
