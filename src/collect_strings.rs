use crate::types::{Decl, Expr};

pub(crate) fn collect_strings<'a>(
    strings: Vec<&'a str>,
    decls: &[Decl<'a, &'a str>],
) -> (Vec<&'a str>, Vec<Decl<'a, usize>>) {
    match decls {
        [decl, rest @ ..] => {
            let (strings, decl) = collect_strings_decl(strings, decl);
            let (strings, rest) = collect_strings(strings, rest);

            let mut decls = vec![decl];
            decls.extend(rest);
            (strings, decls)
        }
        [] => (strings, vec![]),
    }
}

fn collect_strings_decl<'a>(
    strings: Vec<&'a str>,
    decl: &Decl<'a, &'a str>,
) -> (Vec<&'a str>, Decl<'a, usize>) {
    match decl {
        Decl::Bind(symbol, args, expr) => {
            let (strings, expr) = collect_strings_expr(strings, expr);
            (strings, Decl::Bind(symbol, args.to_owned(), expr))
        }
    }
}

fn collect_strings_expr<'a>(
    mut strings: Vec<&'a str>,
    expr: &Expr<&'a str>,
) -> (Vec<&'a str>, Expr<usize>) {
    match expr {
        Expr::Binop(op, lhs, rhs) => {
            let (strings, lhs) = collect_strings_expr(strings, lhs);
            let (strings, rhs) = collect_strings_expr(strings, rhs);
            (strings, Expr::Binop(*op, Box::new(lhs), Box::new(rhs)))
        }
        Expr::Identifier(value) => (strings, Expr::Identifier(value.to_owned())),
        Expr::Integer(value) => (strings, Expr::Integer(*value)),
        Expr::String(value) => {
            strings.push(value);
            let index = strings.len() - 1;
            (strings, Expr::String(index))
        }
    }
}
