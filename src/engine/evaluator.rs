use super::Instruction;
use crate::helper::safe_add;
use std::{
    // collections::VecDeque,
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum EvalError {
    PCOverFlow,
    SPOverFlow,
    // 以下の2つは評価器にエラーがあるときに発生する
    InvalidPC,
    // InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

impl Error for EvalError {}

fn eval_depth(
    inst: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
    backward_match: bool,
) -> Result<bool, EvalError> {
    // 文字数。Match到達の際、backward_matchが有効だったらばspがlast_indexと一致しているはず
    let last_index = line.len();
    loop {
        let next = if let Some(i) = inst.get(pc) {
            i
        } else {
            return Err(EvalError::InvalidPC);
        };

        println!("inst: {:?}", inst);
        println!("line: {:?}", line);

        match next {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Dot => {
                // dotのときは、対象の文字があれば良いので、文字があるならpcとspをインクリメント
                safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                safe_add(&mut sp, &1, || EvalError::SPOverFlow)?;
            }
            Instruction::Match => {
                if last_index != sp && backward_match {
                    return Ok(false);
                }
                return Ok(true);
            }
            Instruction::Jump(addr) => {
                // jumpでは入力の値でpcを更新する
                pc = *addr;
            }
            Instruction::Split(addr1, addr2) => {
                if eval_depth(inst, line, *addr1, sp, backward_match)?
                    || eval_depth(inst, line, *addr2, sp, backward_match)?
                {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}

/// Instructionの配列を受けて、line(入力文字列)にmatchしたらtrue、しなければfalse、例外時はEvalErrorを返す
/// is_depthは有効の時深さ優先探索、無効の時幅優先探索(実装はTODO)を行う
pub fn eval(
    inst: &[Instruction],
    line: &[char],
    is_depth: bool,
    backward_match: bool,
) -> Result<bool, EvalError> {
    if is_depth {
        eval_depth(inst, line, 0, 0, backward_match)
    } else {
        Ok(false)
        // TODO
        // eval_width();
    }
}
