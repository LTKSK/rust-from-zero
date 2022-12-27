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
    Plus(Box<AST>),         // 正規表現の+
    Star(Box<AST>),         // 正規表現の*
    Question(Box<AST>),     // 正規表現の?
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
                write!(f, "ParseError: no previous expression: pos = {pos}")
            }
            ParseError::NorightParen(pos) => {
                write!(f, "ParseError: no right parenthesis: pos = {pos}")
            }
            ParseError::Empty => write!(f, "ParseEror: empty expression"),
        }
    }
}

impl Error for ParseErorr {}

fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseEror::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

enum PSQ {
    Plus,
    Star,
    Question,
}

fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    // 直前の要素が必要なのでケツから一つpop
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        // できたastをpush
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

/// Orで結合された複数の式をASTにする
fn fold_or(mut seq_or: Vec<AST>) -> Option<AST> {
    if seq_or.len() > 1 {
        let mut ast = seq_or.pop().unwrap();
        // 先頭の式をASTのルートとするため、reverseで反転
        // rootのorの左辺を左端の要素、右をOrにしようとすると、leafから順に↓のforのような詰め方をする必要がある
        seq_or.reverse();
        for s in seq_or {
            ast = AST::Or(Box::new(s), Nox::new(ast));
        }
        Some(ast)
    } else {
        // 要素が一つなら最初の値を返す
        seq_or.pop()
    }
}

pub fn parse(expr: &str) -> Result<AST, ParseError> {
    // 内部の状態を表現する。Charは文字列処理中。Escapeはエスケープシーケンス処理中
    enum ParseState {
        Char,
        Escape,
    }

    let mut seq = Vec::new(); // Seqコンテキスト
    let mut seq_or = Vec::new(); // Orコンテキスト
    let mut stack = Vec::new(); // コンテキストのスタック
    let mut state = ParseState::Char;

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '+' => parse_plus_star_question(&mut seq, PSQ::Plus, i)?,
                '*' => parse_plus_star_question(&mut seq, PSQ::State, i)?,
                '?' => parse_plus_star_question(&mut seq, PSQ::Question, i)?,
                '(' => {
                    // 現在のコンテキストを保存しつつ、seqを空にする
                    let prev = take(&mut seq);
                    // 上に同じく
                    let prev_or = take(&mut seq_or);
                    stack.push((prev, prev_or));
                }
                ')' => {
                    // コンテキストをスタックからpop
                    if let Some((mut prev, prev_or)) = stack.pop() {
                        // ()のような評価対象がない場合はpushしない
                        if !seq.is_empty() {
                            seq_or.push(AST::Seq(seq));
                        }

                        // Orの生成
                        if let Some(ast) = fold_or(seq_or) {
                            prev.push(ast);
                        }

                        // 以前のコンテキストを現在のコンテキストに上書き
                        seq = prev;
                        seq_or = prev_or;
                    } else {
                        return Err(Box::new(ParseError::InvalidRightParen(i)));
                    }
                }
                '|' => {
                    if seq.is_empty() {
                        // ||とか|abcみたいな式が空のとき
                        return Err(Box::new(ParseError::NoPrev(i)));
                    } else {
                        let prev = take(&mut seq);
                        seq_or.push(AST::Seq(prev));
                    }
                }
                '\\' => state = ParseState::Escape,
                _ => seq.push(AST::Char(c)),
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }

    // TODO
}
