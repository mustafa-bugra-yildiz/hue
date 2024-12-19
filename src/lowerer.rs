use crate::types::{Ctx, Decl, Expr, Fn, Inst, VReg};

pub(crate) fn lower<'a>(strings: Vec<&'a str>, decls: Vec<Decl<'a, usize>>) -> Ctx<'a> {
    let fns: Vec<_> = decls.into_iter().map(lower_decl).collect();
    Ctx::new(strings, fns)
}

fn lower_decl(decl: Decl<usize>) -> Fn {
    match decl {
        Decl::Bind(symbol, args, expr) => {
            let reg = VReg::default();
            let args: Vec<_> = args
                .into_iter()
                .map(|a| reg.with_name(a.to_string()))
                .collect();
            let (reg, insts) = lower_expr(&args, reg, expr);
            Fn::new(symbol, args, insts, reg)
        }
    }
}

fn lower_expr(args: &[VReg], reg: VReg, expr: Expr<usize>) -> (VReg, Vec<Inst>) {
    match expr {
        Expr::Binop(op, lhs, rhs) => {
            let (lhs_reg, lhs_insts) = lower_expr(args, reg, *lhs);
            let (rhs_reg, rhs_insts) = lower_expr(args, lhs_reg.succ(), *rhs);
            let reg = rhs_reg.succ();
            let insts = vec![]
                .into_iter()
                .chain(lhs_insts)
                .chain(rhs_insts)
                .chain(vec![Inst::Binop(reg.clone(), op, lhs_reg, rhs_reg)])
                .collect();
            (reg, insts)
        }
        Expr::Identifier(value) => (
            args.iter().find(|a| a.is_named(&value)).cloned().unwrap(),
            vec![],
        ),
        Expr::Integer(value) => (reg.clone(), vec![Inst::Mov(reg, value)]),
        Expr::String(index) => (reg.clone(), vec![Inst::Adr(reg, index)]),
    }
}
