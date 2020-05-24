use std::fmt;
use std::fmt::Debug;
use std::io::Error as IOError;
// use std::process;

pub const FROM_IO_ERROR: i32 = 1;
pub const INVALID_ARGS_ERROR: i32 = 2;
pub const OPEN_FILE_ERROR: i32 = 3;
pub const UNDEF_PARSE_ERROR: i32 = 4;

const ERROR_STRINGS: [&str; 4] = [
    "IO Error",
    "Cannot combine flags",
    "Failed to open file",
    "Undefined Parsing Error",
];
#[derive(Debug)]
pub struct Error {
    msg: &'static str,
    //TODO: info should be an enum
    info: Option<String>,
    //TODO: source (underlying error)
    pub code: i32,
}

impl Error {
    pub fn new(n: i32, info: Option<String>) -> Error {
        return Error {
            msg: ERROR_STRINGS[n as usize - 1],
            info: info,
            // source: None, // TODO:
            code: n,
        };
    }
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.info {
            None => write!(f, "{}", self.msg),
            Some(i) => write!(f, "{}: {}", self.msg, i),
        }
    }
}

impl std::convert::From<IOError> for Error {
    fn from(ioerr: IOError) -> Self {
        Error {
            msg: ERROR_STRINGS[0],
            info: Some(format!("{}", ioerr)),
            code: FROM_IO_ERROR,
        }
    }
}

// TODO: is not an official feature yet
//      - https://github.com/rust-lang/rust/issues/42327
//      - https://doc.rust-lang.org/std/option/struct.NoneError.html
// use std::option::NoneError;
// impl std::convert::From<NoneError> for Error {
//     fn from(ioerr: NoneError) -> Self {
//         Error {
//             msg: ERROR_STRINGS[0],
//             info: Some(format!("{}", ioerr)),
//             code: FROM_IO_ERROR,
//         }
//     }
// }
// impl fmt::Debug for Error {}
