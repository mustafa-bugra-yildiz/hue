use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{char, multispace0, space0},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    let prog = args.next().ok_or_else(|| Error::ProgNotFound)?;

    match args.next() {
        Some(file) => {
            let code = std::fs::read_to_string(file).map_err(Error::CannotReadFile)?;
            let (rest, ast) = parse(&code).map_err(Error::Parsing)?;
            if !rest.is_empty() {
                eprintln!("-- REST --\n{rest}\n");
            }

            eprintln!("-- AST --\n{ast:#?}\n");

            let (strings, ast) = collect_strings(vec![], &ast);
            eprintln!("-- STRINGS --\n{strings:#?}\n");

            let ctx = lower(strings, ast);
            eprintln!("-- LOWERING --\n{ctx}\n");

            let ctx = regalloc(ctx)?;
            eprintln!("-- REGALLOC --\n");
            println!("{ctx}");
        }
        None => {
            println!("Usage: {prog} <file>");
        }
    }

    Ok(())
}

// Regalloc Stage

fn regalloc(ctx: Ctx) -> Result<Ctx<Reg>, Error> {
    let fns: Result<Vec<_>, _> = ctx.fns.into_iter().map(regalloc_fn).collect();
    Ok(Ctx {
        strings: ctx.strings,
        fns: fns?,
    })
}

fn regalloc_fn(f: Fn) -> Result<Fn<Reg>, Error> {
    let usage_counts = collect_usage_counts(&f.insts);
    let mut allocator = RegAlloc::new(usage_counts);

    let mut alloced = Vec::new();
    let insts_len = f.insts.len();
    for (i, inst) in f.insts.into_iter().enumerate() {
        let inst = match inst {
            Inst::Binop(dst, op, lhs, rhs) => {
                let rhs = allocator.use_(rhs)?;
                let lhs = allocator.use_(lhs)?;
                let dst = allocator.alloc(dst)?;
                Inst::Binop(dst, op, lhs, rhs)
            }
            Inst::Mov(dst, val) => {
                let dst = allocator.alloc(dst)?;
                Inst::Mov(dst, val)
            }
            Inst::Adr(dst, idx) => {
                let dst = allocator.alloc(dst)?;
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

    Ok(Fn {
        name: f.name,
        insts: alloced,
        ret: Reg::X0,
    })
}

fn collect_usage_counts(insts: &[Inst]) -> HashMap<VReg, usize> {
    let mut usage_counts = HashMap::new();

    for i in insts {
        match i {
            Inst::Binop(_, _, lhs, rhs) => {
                *usage_counts.entry(*lhs).or_insert(0) += 1;
                *usage_counts.entry(*rhs).or_insert(0) += 1;
            }
            Inst::Mov(_, _) => {}
            Inst::Adr(_, _) => {}
        }
    }

    usage_counts
}

// Lowering Stage

fn lower<'a>(strings: Vec<&'a str>, decls: Vec<Decl<'a, usize>>) -> Ctx<'a> {
    let fns: Vec<_> = decls.into_iter().map(lower_decl).collect();
    Ctx { strings, fns }
}

fn lower_decl(decl: Decl<usize>) -> Fn {
    match decl {
        Decl::Bind(symbol, expr) => {
            let (reg, insts) = lower_expr(VReg(0), expr);
            Fn {
                name: symbol,
                insts,
                ret: reg,
            }
        }
    }
}

fn lower_expr(reg: VReg, expr: Expr<usize>) -> (VReg, Vec<Inst>) {
    match expr {
        Expr::Binop(op, lhs, rhs) => {
            let (lhs_reg, lhs_insts) = lower_expr(reg.succ(), *lhs);
            let (rhs_reg, rhs_insts) = lower_expr(lhs_reg.succ(), *rhs);
            let reg = rhs_reg.succ();
            let insts = vec![]
                .into_iter()
                .chain(lhs_insts)
                .chain(rhs_insts)
                .chain(vec![Inst::Binop(reg, op, lhs_reg, rhs_reg)])
                .collect();
            (reg, insts)
        }
        Expr::Integer(value) => (reg, vec![Inst::Mov(reg, value)]),
        Expr::String(index) => (reg, vec![Inst::Adr(reg, index)]),
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
        Expr::Binop(op, lhs, rhs) => {
            let (strings, lhs) = collect_strings_expr(strings, lhs);
            let (strings, rhs) = collect_strings_expr(strings, rhs);
            (strings, Expr::Binop(*op, Box::new(lhs), Box::new(rhs)))
        }
        Expr::Integer(value) => (strings, Expr::Integer(*value)),
        Expr::String(value) => {
            strings.push(value);
            let index = strings.len() - 1;
            (strings, Expr::String(index))
        }
    }
}

// Parsing Stage

fn parse(i: &str) -> IResult<&str, Vec<Decl<&str>>, nom::error::Error<String>> {
    let inner = || -> IResult<&str, Vec<Decl<&str>>, nom::error::Error<&str>> {
        let (i, decls) = many0(parse_decl)(i)?;
        let (i, _) = multispace0(i)?;
        Ok((i, decls))
    };
    let result = inner();
    result.map_err(|e| e.map_input(|i| i.to_string()))
}

fn parse_decl(i: &str) -> IResult<&str, Decl<&str>> {
    let (i, symbol) = lex_symbol(i)?;
    let (i, _) = delimited(space0, tag("="), space0)(i)?;
    let (i, expr) = parse_expr(i)?;
    Ok((i, Decl::Bind(symbol, expr)))
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
    alt((parse_integer, parse_string))(i)
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
    take_while1(|c: char| c.is_alphanumeric())(i)
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
        self.vreg_to_reg.insert(vreg, reg);

        let usage_count = self.usage_counts.get(&vreg).cloned().unwrap_or(0);
        let should_keep_alive = usage_count != 0;
        if should_keep_alive {
            self.free_regs.pop();
        }

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

#[derive(Debug)]
struct Ctx<'a, R = VReg> {
    strings: Vec<&'a str>,
    fns: Vec<Fn<'a, R>>,
}

impl<'a> std::fmt::Display for Ctx<'a, Reg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.strings.iter().enumerate() {
            write!(f, ".lit{idx}: .ascii \"{val}\"\n")?;
        }
        for fn_ in self.fns.iter() {
            write!(f, "{}\n", fn_)?;
        }
        Ok(())
    }
}

impl<'a> std::fmt::Display for Ctx<'a, VReg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.strings.iter().enumerate() {
            write!(f, ".lit{idx}: .ascii \"{val}\"\n")?;
        }
        for fn_ in self.fns.iter() {
            write!(f, "{}\n", fn_)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Fn<'a, R = VReg> {
    name: &'a str,
    insts: Vec<Inst<R>>,
    ret: R,
}

impl<'a> std::fmt::Display for Fn<'a, Reg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n", self.name)?;
        for i in self.insts.iter() {
            write!(f, "  {}\n", i)?;
        }
        write!(f, "  ret")?;
        Ok(())
    }
}

impl<'a> std::fmt::Display for Fn<'a, VReg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n", self.name)?;
        for i in self.insts.iter() {
            write!(f, "  {}\n", i)?;
        }
        write!(f, "  ret {}", self.ret)?;
        Ok(())
    }
}

#[derive(Debug)]
enum Inst<R = VReg> {
    Binop(R, Binop, R, R),
    Mov(R, i64),
    Adr(R, usize),
}

impl<R> Inst<R> {
    fn with_dst(self, reg: R) -> Self {
        match self {
            Inst::Binop(_, op, lhs, rhs) => Inst::Binop(reg, op, lhs, rhs),
            Inst::Mov(_, val) => Inst::Mov(reg, val),
            Inst::Adr(_, idx) => Inst::Adr(reg, idx),
        }
    }
}

impl std::fmt::Display for Inst<VReg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Binop(dst, op, lhs, rhs) => write!(
                f,
                "{dst} = {} {lhs}, {rhs}",
                match op {
                    Binop::Add => "add",
                    Binop::Sub => "sub",
                }
            ),
            Inst::Mov(dst, val) => write!(f, "{dst} = int #{val}"),
            Inst::Adr(dst, idx) => write!(f, "{dst} = adr .lit{idx}"),
        }
    }
}

impl std::fmt::Display for Inst<Reg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inst::Binop(dst, op, lhs, rhs) => write!(
                f,
                "{} {dst}, {lhs}, {rhs}",
                match op {
                    Binop::Add => "add",
                    Binop::Sub => "sub",
                }
            ),
            Inst::Mov(dst, val) => write!(f, "mov {dst}, #{val}"),
            Inst::Adr(dst, idx) => write!(f, "adr {dst}, .lit{idx}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Reg {
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

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct VReg(usize);

impl std::fmt::Display for VReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VReg(idx) => write!(f, "%{idx}"),
        }
    }
}

impl VReg {
    fn succ(&self) -> VReg {
        match self {
            VReg(idx) => VReg(idx + 1),
        }
    }
}

#[derive(Debug)]
enum Decl<'a, S> {
    Bind(&'a str, Expr<S>),
}

#[derive(Debug)]
enum Expr<S> {
    Binop(Binop, Box<Expr<S>>, Box<Expr<S>>),
    Integer(i64),
    String(S),
}

#[derive(Debug, Clone, Copy)]
enum Binop {
    Add,
    Sub,
}

#[derive(Debug)]
enum Error {
    // System errors
    ProgNotFound,

    // IO errors
    CannotReadFile(std::io::Error),

    // Parsing errors
    Parsing(nom::Err<nom::error::Error<String>>),

    // Regalloc errors
    OutOfRegs,
    UnallocedVReg,
    OverusedVReg,
}
