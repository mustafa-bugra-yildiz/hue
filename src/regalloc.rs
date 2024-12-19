use crate::types::{Ctx, Error, Fn, Inst, Reg, VReg};
use std::collections::HashMap;

pub(crate) fn regalloc(ctx: Ctx) -> Result<Ctx<Reg>, Error> {
    let (strings, fns) = ctx.consume();
    let fns: Result<Vec<_>, _> = fns.into_iter().map(regalloc_fn).collect();
    Ok(Ctx::new(strings, fns?))
}

fn regalloc_fn(f: Fn) -> Result<Fn<Reg>, Error> {
    let (f, usage_counts) = collect_usage_counts(f);
    let (name, args, insts, _) = f.consume();

    let mut allocator = RegAlloc::new(usage_counts);

    let args: Vec<_> = args
        .into_iter()
        .enumerate()
        .map(|(i, a)| allocator.alloc_arg(a, i))
        .flatten()
        .collect();

    let mut alloced = Vec::new();
    let insts_len = insts.len();
    for (i, inst) in insts.into_iter().enumerate() {
        let inst = match inst {
            Inst::Binop(dst, op, lhs, rhs) => {
                let rhs = allocator.use_(rhs.clone())?;
                let lhs = allocator.use_(lhs.clone())?;
                let dst = allocator.alloc(dst.clone())?;
                Inst::Binop(dst, op, lhs, rhs)
            }
            Inst::Mov(dst, val) => {
                let dst = allocator.alloc(dst.clone())?;
                Inst::Mov(dst, val)
            }
            Inst::Adr(dst, idx) => {
                let dst = allocator.alloc(dst.clone())?;
                Inst::Adr(dst, idx)
            }
        };

        let is_last = i == insts_len - 1;
        alloced.push(if is_last {
            inst.with_dst(Reg::X0)
        } else {
            inst
        });
    }

    Ok(Fn::new(name, args, alloced, Reg::X0))
}

fn collect_usage_counts(f: Fn) -> (Fn, HashMap<VReg, usize>) {
    let (name, args, insts, ret) = f.consume();
    let mut usage_counts = HashMap::new();

    for (index, arg) in args.iter().enumerate() {
        usage_counts.insert(arg.clone(), index);
    }

    for i in &insts {
        match i {
            Inst::Binop(_, _, lhs, rhs) => {
                *usage_counts.entry(lhs.clone()).or_insert(0) += 1;
                *usage_counts.entry(rhs.clone()).or_insert(0) += 1;
            }
            Inst::Mov(_, _) => {}
            Inst::Adr(_, _) => {}
        }
    }

    (Fn::new(name, args, insts, ret), usage_counts)
}

// Types

struct RegAlloc {
    free_regs: Vec<Reg>,
    vreg_to_reg: HashMap<VReg, Reg>,
    usage_counts: HashMap<VReg, usize>,
}

impl RegAlloc {
    fn new(usage_counts: HashMap<VReg, usize>) -> Self {
        Self {
            free_regs: vec![Reg::X8, Reg::X9, Reg::X10, Reg::X11]
                .into_iter()
                .rev()
                .collect(),
            vreg_to_reg: HashMap::new(),
            usage_counts,
        }
    }

    fn alloc(&mut self, vreg: VReg) -> Result<Reg, Error> {
        let reg = self.free_regs.last().cloned().ok_or(Error::OutOfRegs)?;
        self.vreg_to_reg.insert(vreg.clone(), reg);

        let usage_count = self.usage_counts.get(&vreg).cloned().unwrap_or(0);
        let should_keep_alive = usage_count != 0;
        if should_keep_alive {
            self.free_regs.pop();
        }

        Ok(reg)
    }

    // TODO: Handle more args
    fn alloc_arg(&mut self, vreg: VReg, index: usize) -> Result<Reg, Error> {
        let reg = match index {
            0 => Reg::X0,
            1 => Reg::X1,
            _ => return Err(Error::OutOfRegs),
        };
        self.vreg_to_reg.insert(vreg.clone(), reg);
        Ok(reg)
    }

    fn use_(&mut self, vreg: VReg) -> Result<Reg, Error> {
        let reg = self
            .vreg_to_reg
            .get(&vreg)
            .cloned()
            .ok_or(Error::UnallocedVReg)?;

        if let Some(usage_count) = self.usage_counts.get_mut(&vreg) {
            let is_overused = *usage_count == 0;
            if is_overused {
                return Err(Error::OverusedVReg);
            }

            *usage_count -= 1;

            let should_free = *usage_count == 0;
            if should_free {
                self.free_regs.push(reg);
            }
        }

        Ok(reg)
    }
}
