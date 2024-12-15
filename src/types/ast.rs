use super::Binop;

pub(crate) enum Decl<'a, S> {
    Bind(&'a str, Expr<S>),
}

impl<'a, S: std::fmt::Display> std::fmt::Display for Decl<'a, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decl::Bind(symbol, expr) => {
                write!(f, "fn {symbol}\n")?;
                for line in format!("{expr}").lines() {
                    write!(f, "  {line}\n")?;
                }
                Ok(())
            }
        }
    }
}

pub(crate) enum Expr<S> {
    Binop(Binop, Box<Expr<S>>, Box<Expr<S>>),
    Integer(i64),
    String(S),
}

impl<S: std::fmt::Display> std::fmt::Display for Expr<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binop(op, lhs, rhs) => {
                writeln!(f, "binop {op}")?;
                for line in format!("{lhs}").lines() {
                    writeln!(f, "  {line}")?;
                }
                for line in format!("{rhs}").lines() {
                    writeln!(f, "  {line}")?;
                }
                Ok(())
            }
            Expr::Integer(val) => write!(f, "int {val}"),
            Expr::String(val) => write!(f, "string '{val}'"),
        }
    }
}
