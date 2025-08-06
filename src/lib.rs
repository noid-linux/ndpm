mod args;
mod error;
mod xbps;

pub use crate::args::*;
pub use crate::error::*;
pub use crate::xbps::*;

pub fn is_root() -> bool {
    if let Ok(output) = std::process::Command::new("id").arg("-u").output() {
        if let Ok(uid_str) = String::from_utf8(output.stdout) {
            return uid_str.trim() == "0";
        }
    }
    false
}
