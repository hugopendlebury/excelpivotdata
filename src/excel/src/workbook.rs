use std::io::BufReader;
use std::io::{Read, Seek};

use zip::read::ZipArchive;

use async_zip::ZipFile;

use super::errors::*;
use super::reader::Reader;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PivotCacheDefinition {
    pub cache_defintion_name: String,
    /// Shared strings
    pub(crate) cache_defintion_string: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PivotTable<RS> {
    pub(crate) zip: ZipArchive<RS>,
    /// cache defintions (basically shared strings)
    pub cache_definitions: Vec<PivotCacheDefinition>,
    pub row_data: Vec<Vec<String>>,
}

pub struct Xlsx<RS> {
    pub(crate) zip: ZipArchive<RS>,
    /// Shared strings
    pub(crate) strings: Vec<String>,
    /// Sheets paths
    sheets: Vec<(String, String)>,
}

/// Convenient function to open a file with a BufReader<File>
pub fn open_workbook<R, P>(path: P) -> Result<R, R::Error>
where
    R: Reader<BufReader<File>>,
    P: AsRef<Path>,
{
    let file = BufReader::new(File::open(path)?);
    R::new(file)
}

/// Convenient function to open a file with a BufReader<File>
pub fn open_pivottable<R, P>(path: P) -> Result<R, R::Error>
where
    R: Reader<BufReader<File>>,
    P: AsRef<Path>,
{
    let file = BufReader::new(File::open(path)?);
    R::new(file)
}

impl<RS: Read + Seek> Reader<RS> for Xlsx<RS> {
    type Error = XlsxError;

    fn new(reader: RS) -> Result<Self, XlsxError> {
        let xlsx = Xlsx {
            zip: ZipArchive::new(reader)?,
            strings: Vec::new(),
            sheets: Vec::new(),
        };

        //xlsx.read_pivot_cache()?;
        Ok(xlsx)
    }
}

impl<RS: Read + Seek> Reader<RS> for PivotTable<RS> {
    type Error = XlsxError;

    fn new(reader: RS) -> Result<Self, XlsxError> {
        let mut pt = PivotTable {
            zip: ZipArchive::new(reader)?,
            /// cache defintions (basically shared strings)
            cache_definitions: Vec::new(),
            row_data: Vec::new(),
        };
        pt.read_pivot_cache_defintion()?;
        pt.read_pivot_cache()?;
        Ok(pt)
    }
}
