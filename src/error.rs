use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    XbpsFailed(i32),
    PermissionDenied,

    #[from]
    Zap(zap_rs::Error),

    #[from]
    Io(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        match self {
            Error::XbpsFailed(code) => write!(fmt, "XBPS failed with exit code {code}"),
            Error::PermissionDenied => write!(fmt, "Avoid running ndpm as root/sudo."),
            Error::Zap(e) => write!(fmt, "Zap error: {e}"),
            Error::Io(e) => write!(fmt, "IO error: {e}"),
        }
    }
}

impl std::error::Error for Error {}
