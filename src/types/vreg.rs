#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub(crate) enum VReg {
    Named(String, usize),
    Indexed(usize),
}

impl VReg {
    pub(crate) fn succ(&self) -> VReg {
        match self {
            VReg::Named(_, idx) => VReg::Indexed(*idx),
            VReg::Indexed(idx) => VReg::Indexed(*idx + 1),
        }
    }

    pub(crate) fn with_name(&self, name: String) -> VReg {
        match self {
            VReg::Named(_, idx) => VReg::Named(name, *idx),
            VReg::Indexed(idx) => VReg::Named(name, *idx),
        }
    }

    pub(crate) fn is_named(&self, name: &str) -> bool {
        match self {
            VReg::Named(n, _) => n == name,
            VReg::Indexed(_) => false,
        }
    }
}

impl std::default::Default for VReg {
    fn default() -> Self {
        VReg::Indexed(0)
    }
}

impl std::fmt::Display for VReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VReg::Named(name, idx) => write!(f, "%{name}_{idx}"),
            VReg::Indexed(idx) => write!(f, "%{idx}"),
        }
    }
}
