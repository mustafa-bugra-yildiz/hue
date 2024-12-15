#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) struct VReg(usize);

impl VReg {
    pub(crate) fn succ(&self) -> VReg {
        match self {
            VReg(idx) => VReg(idx + 1),
        }
    }
}

impl std::default::Default for VReg {
    fn default() -> Self {
        VReg(0)
    }
}

impl std::fmt::Display for VReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VReg(idx) => write!(f, "%{idx}"),
        }
    }
}
