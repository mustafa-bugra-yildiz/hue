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

                    eprintln!("-- ASM --");
                    println!("{}", lower(ast));
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

fn lower(decls: Vec<Decl>) -> String {
    let lowered_decls: Vec<_> = decls.into_iter().map(lower_decl).flatten().collect();
    lowered_decls.join("\n")
}

fn lower_decl(decl: Decl) -> Vec<String> {
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

fn lower_expr(expr: Expr) -> Vec<String> {
    match expr {
        Expr::String(value) => {
            vec![format!("  load '{value}'")]
        }
    }
}

fn parse(i: &str) -> IResult<&str, Vec<Decl>> {
    let (i, decls) = many0(parse_decl)(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, decls))
}

fn parse_decl(i: &str) -> IResult<&str, Decl> {
    let (i, symbol) = lex_symbol(i)?;
    let (i, _) = delimited(space0, tag("="), space0)(i)?;
    let (i, expr) = parse_expr(i)?;
    Ok((i, Decl::Bind(symbol, expr)))
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, value) = lex_string(i)?;
    Ok((i, Expr::String(value)))
}

fn lex_symbol(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric())(i)
}

fn lex_string(i: &str) -> IResult<&str, &str> {
    let (i, pair) = char('"')(i)?;
    let (i, value) = take_till(|c| c == pair)(i)?;
    Ok((&i[1..], value))
}

#[derive(Debug)]
enum Decl<'a> {
    Bind(&'a str, Expr<'a>),
}

#[derive(Debug)]
enum Expr<'a> {
    String(&'a str),
}

#[derive(Debug)]
enum Error<'a> {
    ProgNotFound,
    CannotReadFile(std::io::Error),
    Parsing(nom::Err<nom::error::Error<&'a str>>),
}
