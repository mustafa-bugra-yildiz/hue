use crate::types::{Ctx, Decl, Expr, Fn, Inst, VReg};

pub(crate) fn lower<'a>(strings: Vec<&'a str>, decls: Vec<Decl<'a, usize>>) -> Ctx<'a> {
    let fns: Vec<_> = decls.into_iter().map(lower_decl).collect();
    Ctx::new(strings, fns)
}

fn lower_decl(decl: Decl<usize>) -> Fn {
    match decl {
        Decl::Bind(symbol, expr) => {
            let (reg, insts) = lower_expr(VReg::default(), expr);
            Fn::new(symbol, insts, reg)
        }
    }
}

fn lower_expr(reg: VReg, expr: Expr<usize>) -> (VReg, Vec<Inst>) {
    match expr {
        Expr::Binop(op, lhs, rhs) => {
            let (lhs_reg, lhs_insts) = lower_expr(reg, *lhs);
            let (rhs_reg, rhs_insts) = lower_expr(lhs_reg.succ(), *rhs);
            let reg = rhs_reg.succ();
            let insts = vec![]
                .into_iter()
                .chain(lhs_insts)
                .chain(rhs_insts)
                .chain(vec![Inst::Binop(reg, op, lhs_reg, rhs_reg)])
                .collect();
            (reg, insts)
        }
        Expr::Integer(value) => (reg, vec![Inst::Mov(reg, value)]),
        Expr::String(index) => (reg, vec![Inst::Adr(reg, index)]),
    }
}
