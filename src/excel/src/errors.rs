/// An enum for Xlsx specific errors
#[derive(Debug)]
pub enum XlsxError {
    /// Io error
    Io(std::io::Error),
    /// Zip error
    Zip(zip::result::ZipError),
    /// Xml error
    Xml(quick_xml::Error),
    /// Xml attribute error
    XmlAttr(quick_xml::events::attributes::AttrError),
    /// Parse error
    Parse(std::string::ParseError),
    /// Float error
    ParseFloat(std::num::ParseFloatError),
    /// ParseInt error
    ParseInt(std::num::ParseIntError),

    /// Unexpected end of xml
    XmlEof(&'static str),
    /// Unexpected node
    UnexpectedNode(&'static str),
    /// File not found
    FileNotFound(String),
    /// Relationship not found
    RelationshipNotFound,
    /// Expecting alphanumeric character
    Alphanumeric(u8),
    /// Numeric column
    NumericColumn(u8),
    /// Wrong dimension count
    DimensionCount(usize),
    /// Cell 't' attribute error
    CellTAttribute(String),
    /// Cell 'r' attribute error
    CellRAttribute,
    /// Unexpected error
    Unexpected(&'static str),
    /// Cell error
    CellError(String),
}

macro_rules! from_err {
    ($from:ty, $to:tt, $var:tt) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                $to::$var(e)
            }
        }
    };
}

from_err!(std::io::Error, XlsxError, Io);
from_err!(zip::result::ZipError, XlsxError, Zip);
from_err!(quick_xml::Error, XlsxError, Xml);
from_err!(std::string::ParseError, XlsxError, Parse);
from_err!(std::num::ParseFloatError, XlsxError, ParseFloat);
from_err!(std::num::ParseIntError, XlsxError, ParseInt);

impl std::fmt::Display for XlsxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XlsxError::Io(e) => write!(f, "I/O error: {}", e),
            XlsxError::Zip(e) => write!(f, "Zip error: {}", e),
            XlsxError::Xml(e) => write!(f, "Xml error: {}", e),
            XlsxError::XmlAttr(e) => write!(f, "Xml attribute error: {}", e),
            XlsxError::Parse(e) => write!(f, "Parse string error: {}", e),
            XlsxError::ParseInt(e) => write!(f, "Parse integer error: {}", e),
            XlsxError::ParseFloat(e) => write!(f, "Parse float error: {}", e),

            XlsxError::XmlEof(e) => write!(f, "Unexpected end of xml, expecting '</{}>'", e),
            XlsxError::UnexpectedNode(e) => write!(f, "Expecting '{}' node", e),
            XlsxError::FileNotFound(e) => write!(f, "File not found '{}'", e),
            XlsxError::RelationshipNotFound => write!(f, "Relationship not found"),
            XlsxError::Alphanumeric(e) => {
                write!(f, "Expecting alphanumeric character, got {:X}", e)
            }
            XlsxError::NumericColumn(e) => write!(
                f,
                "Numeric character is not allowed for column name, got {}",
                e
            ),
            XlsxError::DimensionCount(e) => {
                write!(f, "Range dimension must be lower than 2. Got {}", e)
            }
            XlsxError::CellTAttribute(e) => write!(f, "Unknown cell 't' attribute: {:?}", e),
            XlsxError::CellRAttribute => write!(f, "Cell missing 'r' attribute"),
            XlsxError::Unexpected(e) => write!(f, "{}", e),
            XlsxError::CellError(e) => write!(f, "Unsupported cell error value '{}'", e),
        }
    }
}

impl std::error::Error for XlsxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            XlsxError::Io(e) => Some(e),
            XlsxError::Zip(e) => Some(e),
            XlsxError::Xml(e) => Some(e),
            XlsxError::Parse(e) => Some(e),
            XlsxError::ParseInt(e) => Some(e),
            XlsxError::ParseFloat(e) => Some(e),
            _ => None,
        }
    }
}
