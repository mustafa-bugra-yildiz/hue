#[derive(Clone, Copy)]
pub(crate) enum Reg {
    X0,
    X8,
    X9,
    X10,
    X11,
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Reg::X0 => "X0",
                Reg::X8 => "X8",
                Reg::X9 => "X9",
                Reg::X10 => "X10",
                Reg::X11 => "X11",
            }
        )
    }
}
