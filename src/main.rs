mod collect_strings;
mod lowerer;
mod parser;
mod regalloc;
mod types;

use types::Error;

fn main() -> Result<(), Error> {
    let mut args = std::env::args();

    let prog = args.next().ok_or_else(|| Error::ProgNotFound)?;

    let Some(file) = args.next() else {
        eprintln!("Usage: {prog} <file>");
        std::process::exit(1);
    };

    let code = std::fs::read_to_string(file).map_err(Error::CannotReadFile)?;
    let (rest, ast) = parser::parse(&code).map_err(Error::Parsing)?;
    if !rest.is_empty() {
        edump("REST", &rest);
    }
    edump_vec("AST", &ast);

    let (strings, ast) = collect_strings::collect_strings(vec![], &ast);
    let ctx = lowerer::lower(strings, ast);
    edump("IR", &ctx);

    let ctx = regalloc::regalloc(ctx)?;
    eprintln!("-- ASM --");
    println!("{ctx}");

    Ok(())
}

fn edump<T: std::fmt::Display>(label: &'static str, v: &T) {
    eprintln!("-- {label} --\n{v}\n")
}

fn edump_vec<T: std::fmt::Display>(label: &'static str, v: &[T]) {
    eprintln!("-- {label} --");
    for item in v {
        eprintln!("{item}");
    }
}
