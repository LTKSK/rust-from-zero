mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

/// ファイルをオープンし、各行にマッチングを行う
/// abcdという文字列があった場合、 abcd -> bcd -> cd -> dの順にマッチが行われる
fn match_file(expr: &str, file_path: &str) -> Result<(), DynError> {
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;
    println!();

    for line in reader.lines() {
        let line = line?;
        // abcdみたいな入力のときは、abcd, bcd, cd ,cのように入力していく
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], true)? {
                println!("{line}");
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        // eprintlnはstderrに吐き出す
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::do_matching,
        helper::{safe_add, SafeAdd},
    };

    #[test]
    fn test_safe_add() {
        let n: usize = 10;
        assert_eq!(Some(30), n.safe_add(&20));

        let n: usize = !0; // 2^64 - 1
        assert_eq!(None, n.safe_add(&1));

        let mut n: usize = 10;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n: usize = !0;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching() {
        // parse error
        assert!(do_matching("+b", "bbb", true).is_err());
        assert!(do_matching("*b", "bbb", true).is_err());
        assert!(do_matching("|b", "bbb", true).is_err());
        assert!(do_matching("?b", "bbb", true).is_err());
        assert!(do_matching("(abc", "bbb", true).is_err());
        assert!(do_matching("abc)", "bbb", true).is_err());

        // parse成功でマッチ成功
        assert!(do_matching("abc|def", "def", true).unwrap());
        assert!(do_matching("(abc)*", "abcabc", true).unwrap());
        assert!(do_matching("(ab|cd)+", "abcdcd", true).unwrap());
        assert!(do_matching("abc?", "abd", true).unwrap());

        // parse成功でマッチ失敗
        assert!(!do_matching("abc|def", "efa", true).unwrap());
        assert!(!do_matching("(ab|cd)+", "efa", true).unwrap());
        assert!(!do_matching("abc?", "acb", true).unwrap());
    }

    #[test]
    fn test_matching_multi_byte_characters() {
        assert!(do_matching("あいう|えお", "あいう", true).unwrap());
        assert!(do_matching("(ワク)*", "ワクワク", true).unwrap());

        // parse成功でマッチ失敗
        assert!(!do_matching("ほげ|ふが", "失敗", true).unwrap());
        assert!(!do_matching("(ふー|ばー)+", "ばば", true).unwrap());
    }

    #[test]
    fn test_escape文字() {
        assert!(do_matching("\\.あいう", ".あいうえお", true).unwrap());
        assert!(do_matching("\\?あいう", "?あいうえお", true).unwrap());
        assert!(do_matching("\\+あいう", "+あいうえお", true).unwrap());
        assert!(do_matching("\\*あいう", "*あいうえお", true).unwrap());
    }

    #[test]
    fn test_ドットによる任意の1文字のマッチング() {
        assert!(do_matching("あ.か", "あいかえお", true).unwrap());
        assert!(do_matching("か..け", "かきくけこ", true).unwrap());

        // // 失敗パターン
        assert!(!do_matching("い.え", "あいえお", true).unwrap());
        assert!(!do_matching(".あ.", "かきくけこ", true).unwrap());
    }
}
