use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::common::position::Position;
use crate::type_checker::CheckInput;

pub type TypeResult<T = GenericType> = std::result::Result<T, Vec<TypeErr>>;
pub type TypeResults = std::result::Result<Vec<CheckInput>, Vec<TypeErr>>;

#[derive(Debug)]
pub struct TypeErr {
    pub position:    Position,
    pub msg:         String,
    pub path:        Option<PathBuf>,
    pub source_line: Option<String>
}

impl From<TypeErr> for Vec<TypeErr> {
    fn from(type_err: TypeErr) -> Self { vec![type_err] }
}

impl TypeErr {
    pub fn new(position: &Position, msg: &str) -> TypeErr {
        TypeErr {
            position:    position.clone(),
            msg:         String::from(msg),
            path:        None,
            source_line: None
        }
    }

    pub fn into_with_source(self, source: &Option<String>, path: &Option<PathBuf>) -> TypeErr {
        TypeErr {
            position:    self.position.clone(),
            msg:         self.msg.clone(),
            source_line: source.clone().map(|source| {
                source
                    .lines()
                    .nth(self.position.start.line as usize - 1)
                    .map_or(String::from("unknown"), String::from)
            }),
            path:        path.clone()
        }
    }
}

impl Display for TypeErr {
    // Deal with Positions that cover multiple lines
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "--> {}:{}:{}
     | {}
{:3}  |- {}
     |  {}{}",
            self.path.clone().map_or(String::from("<unknown>"), |path| format!("{:#?}", path)),
            self.position.start.line,
            self.position.start.pos,
            self.msg,
            self.position.start.line,
            self.source_line
                .clone()
                .map_or(String::from("<unknown>"), |line| format!("{:#?}", line)),
            String::from_utf8(vec![b' '; self.position.start.pos as usize]).unwrap(),
            String::from_utf8(vec![b'^'; self.position.get_width() as usize]).unwrap()
        )
    }
}
