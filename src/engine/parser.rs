use std::{
    error::Error,
    fmt::{self, Display},
    // 入力の変数から、所有権の取得しその変数の初期化を同時に行う
    mem::take,
};

#[derive(Debug)]
pub enum AST {
    Char(char),
    // 以下の4つは対象となるASTを受ける
    Plus(Box<AST>), // 正規表現の+
    Star(Box<AST>), // 正規表現の*
    Question(Box<AST>), // 正規表現の?
    Or(Box<AST>, Box<AST>), // 正規表現の|
    // 複数のASTをまとめて扱うために使う
    Seq(Vec<AST>),
}

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char), // 誤ったエスケープシーケンス
    InvalidRightParen(usize),   // 開きカッコなし
    NoPrev(usize),              // + | * ? の前に式がない
    NorightParen,               // 閉じカッコなし
    Empty,                      // 空のパターン
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{c}'")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
        }
    }
}