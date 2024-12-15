#[derive(Clone, Copy)]
pub(crate) enum Binop {
    Add,
    Sub,
}

impl std::fmt::Display for Binop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Binop::Add => "+",
                Binop::Sub => "-",
            }
        )
    }
}
