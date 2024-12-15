use super::{Binop, Reg, VReg};

pub(crate) enum Inst<R = VReg> {
    Binop(R, Binop, R, R),
    Mov(R, i64),
    Adr(R, usize),
}

impl<R> Inst<R> {
    pub(crate) fn with_dst(self, reg: R) -> Self {
        match self {
            Inst::Binop(_, op, lhs, rhs) => Inst::Binop(reg, op, lhs, rhs),
            Inst::Mov(_, val) => Inst::Mov(reg, val),
            Inst::Adr(_, idx) => Inst::Adr(reg, idx),
        }
    }
}

impl std::fmt::Display for Inst<Reg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Binop(dst, op, lhs, rhs) => write!(
                f,
                "{} {dst}, {lhs}, {rhs}",
                match op {
                    Binop::Add => "add",
                    Binop::Sub => "sub",
                }
            ),
            Inst::Mov(dst, val) => write!(f, "mov {dst}, #{val}"),
            Inst::Adr(dst, idx) => write!(f, "adr {dst}, .lit{idx}"),
        }
    }
}

impl std::fmt::Display for Inst<VReg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Binop(dst, op, lhs, rhs) => write!(
                f,
                "{dst} = {} {lhs}, {rhs}",
                match op {
                    Binop::Add => "add",
                    Binop::Sub => "sub",
                }
            ),
            Inst::Mov(dst, val) => write!(f, "{dst} = int #{val}"),
            Inst::Adr(dst, idx) => write!(f, "{dst} = adr $lit{idx}"),
        }
    }
}
