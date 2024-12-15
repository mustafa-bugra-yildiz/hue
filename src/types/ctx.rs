use super::{Fn, Reg, VReg};

pub(crate) struct Ctx<'a, R = VReg> {
    strings: Vec<&'a str>,
    fns: Vec<Fn<'a, R>>,
}

impl<'a, R> Ctx<'a, R> {
    pub(crate) fn new(strings: Vec<&'a str>, fns: Vec<Fn<'a, R>>) -> Self {
        Self { strings, fns }
    }

    // NOTE: Is this really necessary?
    pub(crate) fn consume(self) -> (Vec<&'a str>, Vec<Fn<'a, R>>) {
        (self.strings, self.fns)
    }
}

impl<'a> std::fmt::Display for Ctx<'a, Reg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fns: Vec<_> = self.fns.iter().map(|f| format!("{f}")).collect();
        writeln!(f, "{}", fns.join("\n\n"))?;

        let strings: Vec<_> = self
            .strings
            .iter()
            .enumerate()
            .map(|(idx, val)| format!(".lit{idx}: .ascii \"{val}\""))
            .collect();
        if !strings.is_empty() {
            writeln!(f, "\n{}", strings.join("\n\n"))?;
        }

        Ok(())
    }
}

impl<'a> std::fmt::Display for Ctx<'a, VReg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fns: Vec<_> = self.fns.iter().map(|f| format!("{f}")).collect();
        writeln!(f, "{}", fns.join("\n\n"))?;

        let strings: Vec<_> = self
            .strings
            .iter()
            .enumerate()
            .map(|(idx, val)| format!("let $lit{idx} = \"{val}\""))
            .collect();
        if !strings.is_empty() {
            writeln!(f, "\n{}", strings.join("\n\n"))?;
        }

        Ok(())
    }
}
