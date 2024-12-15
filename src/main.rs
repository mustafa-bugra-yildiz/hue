use nom::{
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{char, multispace0, space0},
    multi::many0,
    sequence::delimited,
    IResult,
};

fn main() -> Result<(), Error<'static>> {
    let mut args = std::env::args();
    let prog = args.next().ok_or_else(|| Error::ProgNotFound)?;

    match args.next() {
        Some(file) => {
            let code = std::fs::read_to_string(file).map_err(Error::CannotReadFile)?;
            match parse(&code).map_err(Error::Parsing) {
                Ok((rest, ast)) => {
                    if !rest.is_empty() {
                        eprintln!("-- REST --\n{rest}\n");
                    }

                    eprintln!("-- AST --\n{ast:#?}\n");

                    let (strings, ast) = collect_strings(vec![], &ast);
                    eprintln!("-- STRINGS --\n{strings:#?}\n");

                    eprintln!("-- ASM --");
                    println!("{}", lower(strings, ast));
                }
                Err(e) => {
                    eprintln!("error: {e:#?}");
                }
            }
        }
        None => {
            println!("Usage: {prog} <file>");
        }
    }

    Ok(())
}

// Lowering Stage

fn lower(strings: Vec<&str>, decls: Vec<Decl<usize>>) -> String {
    let lowered_strings: Vec<_> = strings
        .into_iter()
        .enumerate()
        .map(lower_string)
        .flatten()
        .collect();
    let lowered_strings = lowered_strings.join("\n");

    let lowered_decls: Vec<_> = decls.into_iter().map(lower_decl).flatten().collect();
    let lowered_decls = lowered_decls.join("\n");

    vec![lowered_decls, lowered_strings].join("\n\n")
}

fn lower_string((index, value): (usize, &str)) -> Vec<String> {
    vec![format!(".lit{index}: .ascii \"{value}\"")]
}

fn lower_decl(decl: Decl<usize>) -> Vec<String> {
    match decl {
        Decl::Bind(symbol, expr) => {
            let mut out = vec![];

            out.push(format!("{symbol}:"));

            let mut expr = lower_expr(expr);
            out.append(&mut expr);

            out.push(format!("  ret x0"));
            out
        }
    }
}

fn lower_expr(expr: Expr<usize>) -> Vec<String> {
    match expr {
        Expr::String(index) => {
            vec![format!("  adr x0, .lit{index}")]
        }
    }
}

// String Collection Stage

fn collect_strings<'a>(
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
        Decl::Bind(symbol, expr) => {
            let (strings, expr) = collect_strings_expr(strings, expr);
            (strings, Decl::Bind(symbol, expr))
        }
    }
}

fn collect_strings_expr<'a>(
    mut strings: Vec<&'a str>,
    expr: &Expr<&'a str>,
) -> (Vec<&'a str>, Expr<usize>) {
    match expr {
        Expr::String(value) => {
            strings.push(value);
            let index = strings.len() - 1;
            (strings, Expr::String(index))
        }
    }
}

// Parsing Stage

fn parse(i: &str) -> IResult<&str, Vec<Decl<&str>>> {
    let (i, decls) = many0(parse_decl)(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, decls))
}

fn parse_decl(i: &str) -> IResult<&str, Decl<&str>> {
    let (i, symbol) = lex_symbol(i)?;
    let (i, _) = delimited(space0, tag("="), space0)(i)?;
    let (i, expr) = parse_expr(i)?;
    Ok((i, Decl::Bind(symbol, expr)))
}

fn parse_expr(i: &str) -> IResult<&str, Expr<&str>> {
    let (i, value) = lex_string(i)?;
    Ok((i, Expr::String(value)))
}

// Lexing Stage

fn lex_symbol(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(i)
}

fn lex_string(i: &str) -> IResult<&str, &str> {
    let (i, pair) = char('"')(i)?;
    let (i, value) = take_till(|c| c == pair)(i)?;
    Ok((&i[1..], value))
}

// Types

#[derive(Debug)]
enum Decl<'a, S> {
    Bind(&'a str, Expr<S>),
}

#[derive(Debug)]
enum Expr<S> {
    String(S),
}

#[derive(Debug)]
enum Error<'a> {
    ProgNotFound,
    CannotReadFile(std::io::Error),
    Parsing(nom::Err<nom::error::Error<&'a str>>),
}
