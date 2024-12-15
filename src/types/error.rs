pub(crate) enum Error {
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

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ProgNotFound => {
                writeln!(f, "error: Your system does not provide the program path as the first CLI argument.")?;
                writeln!(
                    f,
                    "error: Please report this bug at the source forge we use,"
                )?;
                writeln!(f, "error: Because your system is either a really weird one or you are about to segfault.")?;
                Ok(())
            }
            Error::CannotReadFile(error) => {
                writeln!(f, "error: {error:?}")
            }
            Error::Parsing(error) => {
                writeln!(f, "error: {error:?}")
            }
            Error::OutOfRegs => {
                writeln!(f, "error: out of registers")
            }
            Error::UnallocedVReg => {
                writeln!(f, "error: unalloced vreg used")
            }
            Error::OverusedVReg => {
                writeln!(f, "error: a vreg was overused")
            }
        }
    }
}
