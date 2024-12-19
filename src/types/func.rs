use super::{Inst, Reg, VReg};

pub(crate) struct Fn<'a, R = VReg> {
    name: &'a str,
    args: Vec<R>,
    insts: Vec<Inst<R>>,
    ret: R,
}

impl<'a, R> Fn<'a, R> {
    pub(crate) fn new(name: &'a str, args: Vec<R>, insts: Vec<Inst<R>>, ret: R) -> Self {
        Self {
            name,
            args,
            insts,
            ret,
        }
    }

    // NOTE: Is this really necessary?
    pub(crate) fn consume(self) -> (&'a str, Vec<R>, Vec<Inst<R>>, R) {
        (self.name, self.args, self.insts, self.ret)
    }
}

impl<'a> std::fmt::Display for Fn<'a, Reg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for i in self.insts.iter() {
            writeln!(f, "  {}", i)?;
        }
        write!(f, "  ret")?;
        Ok(())
    }
}

impl<'a> std::fmt::Display for Fn<'a, VReg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "fn ${}({}) {{",
            self.name,
            self.args
                .iter()
                .map(|a| format!("{a}"))
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        for i in self.insts.iter() {
            writeln!(f, "  {}", i)?;
        }
        write!(f, "  ret {}\n}}", self.ret)?;
        Ok(())
    }
}
