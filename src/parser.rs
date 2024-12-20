use crate::types::{Binop, Decl, Expr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{char, multispace0, space0, space1},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

// Parsing Stage

pub(crate) fn parse(i: &str) -> IResult<&str, Vec<Decl<&str>>, nom::error::Error<String>> {
    let inner = || -> IResult<&str, Vec<Decl<&str>>, nom::error::Error<&str>> {
        let (i, decls) = many0(delimited(multispace0, parse_decl, multispace0))(i)?;
        let (i, _) = multispace0(i)?;
        Ok((i, decls))
    };
    let result = inner();
    result.map_err(|e| e.map_input(|i| i.to_string()))
}

fn parse_decl(i: &str) -> IResult<&str, Decl<&str>> {
    let (i, _) = lex_let(i)?;
    let (i, _) = space1(i)?;
    let (i, symbol) = lex_symbol(i)?;

    let (i, _) = space0(i)?;
    let (i, args) = separated_list0(space1, lex_symbol)(i)?;
    let (i, _) = space0(i)?;

    let (i, _) = delimited(space0, tag("="), space0)(i)?;
    let (i, expr) = parse_expr(i)?;
    Ok((i, Decl::Bind(symbol, args, expr)))
}

fn parse_expr(i: &str) -> IResult<&str, Expr<&str>> {
    parse_add(i)
}

fn parse_add(i: &str) -> IResult<&str, Expr<&str>> {
    let (i, lhs) = parse_literal(i)?;
    let (i, rhs) = many0(tuple((
        delimited(space0, lex_addsubop, space0),
        parse_literal,
    )))(i)?;
    let expr = rhs.into_iter().fold(lhs, |lhs, (op, rhs)| {
        Expr::Binop(op, Box::new(lhs), Box::new(rhs))
    });
    Ok((i, expr))
}

fn parse_literal(i: &str) -> IResult<&str, Expr<&str>> {
    alt((parse_ident, parse_integer, parse_string))(i)
}

fn parse_ident(i: &str) -> IResult<&str, Expr<&str>> {
    let (i, value) = lex_symbol(i)?;
    Ok((i, Expr::Identifier(value.to_string())))
}

fn parse_integer(i: &str) -> IResult<&str, Expr<&str>> {
    let (i, value) = lex_integer(i)?;
    Ok((i, Expr::Integer(value)))
}

fn parse_string(i: &str) -> IResult<&str, Expr<&str>> {
    let (i, value) = lex_string(i)?;
    Ok((i, Expr::String(value)))
}

// Lexing Stage

fn lex_let(i: &str) -> IResult<&str, &str> {
    tag("let")(i)
}

fn lex_addsubop(i: &str) -> IResult<&str, Binop> {
    let (i, op) = alt((char('+'), char('-')))(i)?;
    let op = match op {
        '+' => Binop::Add,
        '-' => Binop::Sub,
        _ => unreachable!(),
    };
    Ok((i, op))
}

fn lex_symbol(i: &str) -> IResult<&str, &str> {
    // Check first char is alphabetic or underscore
    i.chars()
        .next()
        .and_then(|c| {
            if c.is_alphabetic() || c == '_' {
                Some(c)
            } else {
                None
            }
        })
        .ok_or(nom::Err::Error(nom::error::Error::new(
            i,
            nom::error::ErrorKind::Eof,
        )))?;

    // Check rest of chars are alphanumeric or underscore
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(i)
}

fn lex_integer(i: &str) -> IResult<&str, i64> {
    let (i, value) = take_while1(|c: char| c.is_digit(10))(i)?;
    Ok((i, value.parse().expect("error parsing integer")))
}

fn lex_string(i: &str) -> IResult<&str, &str> {
    let (i, pair) = char('"')(i)?;
    let (i, value) = take_till(|c| c == pair)(i)?;
    Ok((&i[1..], value))
}
