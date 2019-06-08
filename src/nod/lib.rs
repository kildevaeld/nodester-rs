
#[macro_use]
extern crate serde;

pub mod error;
mod platform;
mod version;
mod path;
mod utils;
mod fetch;
mod node;
mod teewriter;

pub use version::Version;
pub use platform::{Platform, Arch};
pub use error::{Result,NodError};
pub use node::{Node, Progress};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
