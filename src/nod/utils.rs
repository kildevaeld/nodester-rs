

use error::{Result, NodError};
use platform::{Platform, Arch};

pub fn getplatform() -> Result<Platform> {
    if cfg!(target_os = "macos") {
        return Ok(Platform::Darwin);
    } else if cfg!(target_os = "linux") {
        return Ok(Platform::Linux);
    }
    Err(NodError::Other("unsupported os"))
}

pub fn getarch() -> Result<Arch> {
    if cfg!(target_arch = "x86") {
        return Ok(Arch::X86);
    } else if cfg!(any(target_arch = "x86_64", target = "aarch64")) {
        //let pf = try!(getplatform());
        return Ok(Arch::X64);

    }
    Err(NodError::Other("unsupported arch"))
}