use super::{parser::AST, Instruction};
use crate::helper::safe_add;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum CodeGenError {
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}
impl Error for CodeGenError {}

#[derive(Default, Debug)]
struct Generator {
    pc: usize,               //次に生成するアセンブリ命令のアドレス
    insts: Vec<Instruction>, // 命令の一覧。get_codeではこれを返す
}

impl Generator {
    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        // クロージャでエラーを返すようにすると、メモリのアロケーションを遅延することができる
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverFlow)
    }

    fn gen_seq(&mut self, exprs: &[AST]) -> Result<(), CodeGenError> {
        for e in exprs {
            self.gen_expr(e)?;
        }
        Ok(())
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        let inst = Instruction::Char(c);
        self.insts.push(inst);
        self.inc_pc()?;
        Ok(())
    }

    fn gen_question(&mut self, e: &AST) -> Result<(), CodeGenError> {
        // split L1, L2
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0); // self.pcがL1。L2を仮に0と設定
        self.insts.push(split);

        // L1: eのコード
        self.gen_expr(e)?;

        // L2の値を設定
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailQuestion)
        }
    }
    /// ```text
    /// L1: split L2, L3
    /// L2: eのコード
    ///     jump L1
    /// L3:
    /// ```
    fn gen_star(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);

        self.gen_expr(e)?;

        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
            Ok(())
        } else {
            Err(CodeGenError::FailStar)
        }
    }

    /// ```text
    /// L1: eのコード
    ///     split L1, L2
    /// L2:
    /// ```
    fn gen_plus(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let l1 = self.pc;
        self.gen_expr(e)?;

        self.inc_pc()?;
        let split = Instruction::Split(l1, self.pc);
        self.insts.push(split);
        Ok(())
    }

    /// or演算子のコード生成器
    ///     split L1, L2
    /// L1: e1のコード
    ///     jmp L3
    /// L2: e2のコード
    /// L3: ...
    /// のようなコードを生成する
    fn gen_or(&mut self, e1: &AST, e2: &AST) -> Result<(), CodeGenError> {
        let split_addr = self.pc;
        self.inc_pc()?;
        // L1はself.pc、L2は仮で0とする
        // L1はsplitの次のアドレスなのでインクリメントしている
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        // e1を再帰的に解釈
        self.gen_expr(e1)?;

        // jmp L3を生成。l1の終わりにl3へ飛ぶ必要がある（その間にl2の処理が来る）
        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0)); // L3を仮で0とする。e2の解釈後でないと場所がわからないため

        // L2の値を設定。このときのPCの値がl2になるということ
        self.inc_pc()?;
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        // L2: e2のコード
        self.gen_expr(e2)?;

        // 0で埋めていたL3の値をここで設定
        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jmp_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Or(e1, e2) => self.gen_or(e1, e2)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e) => self.gen_star(e)?,
            AST::Question(e) => self.gen_question(e)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }

    fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;
        self.inc_pc()?;
        // 最後にmatchをおいて終わり
        self.insts.push(Instruction::Match);
        Ok(())
    }
}

pub fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}
