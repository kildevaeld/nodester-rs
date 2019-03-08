use std::fmt;

#[derive(Debug)]
pub enum Platform {
    Darwin,
    Linux,
    Unknown,
}

impl Into<String> for Platform {
    fn into(self) -> String {
        match self {
            Platform::Darwin => "darwin".to_string(),
            Platform::Linux => "linux".to_string(),
            Platform::Unknown => "unknown".to_string(),
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Platform::Darwin => write!(f, "darwin"),
            Platform::Linux => write!(f, "linux"),
            Platform::Unknown => write!(f, "unknown")
        }
    }
}

#[derive(Debug)]
pub enum Arch {
    X86,
    X64,
    Unknown,
}

impl Into<String> for Arch {
    fn into(self) -> String {
        match self {
            Arch::X64 => "x64".to_string(),
            Arch::X86 => "x86".to_string(),
            Arch::Unknown => "unknown".to_string(),
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arch::X64 => write!(f, "x64"),
            Arch::X86 => write!(f, "x86"),
            Arch::Unknown => write!(f, "unknown")
        }
    }
}
