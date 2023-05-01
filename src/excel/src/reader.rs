use std::io::BufReader;
use std::io::{Read, Seek};

use quick_xml::Reader as XmlReader;
use zip::read::{ZipArchive, ZipFile};
use zip::result::ZipError;

//use async_zip::ZipFile;

use async_zip::base::read::seek::ZipFileReader;
use tokio::fs::{create_dir_all, File, OpenOptions};
//use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

use super::errors::*;

pub (crate) type XlsReader<'a> = XmlReader<BufReader<ZipFile<'a>>>;

/* 
pub (crate) type XlsReader2<'a> = XmlReader<BufReader<ZipFileReader<dyn Read>>>;


//ZipFileReader::new(archive).await.expect("Failed to read zip file");
pub async fn xml_reader2<'a, RS: Read + Seek>(
    zip: &mut ZipFile,
    path: &str,
) -> Option<Result<XlsReader<'a>, XlsxError>> {
    let mut reader = 
    match zip.by_name(path) {
        Ok(f) => {
            let mut r = XmlReader::from_reader(BufReader::new(f));
            r.check_end_names(false)
                .trim_text(false)
                .check_comments(false)
                .expand_empty_elements(true);
            Some(Ok(r))
        }
        Err(ZipError::FileNotFound) => None,
        Err(e) => Some(Err(e.into())),
    }
}
*/
pub fn xml_reader<'a, RS: Read + Seek>(
    zip: &'a mut ZipArchive<RS>,
    path: &str,
) -> Option<Result<XlsReader<'a>, XlsxError>> {
    match zip.by_name(path) {
        Ok(f) => {
            let mut r = XmlReader::from_reader(BufReader::new(f));
            r.check_end_names(false)
                .trim_text(false)
                .check_comments(false)
                .expand_empty_elements(true);
            Some(Ok(r))
        }
        Err(ZipError::FileNotFound) => None,
        Err(e) => Some(Err(e.into())),
    }
}


pub trait Reader<RS>: Sized
where
RS: Read + Seek,
{
/// Error specific to file type
type Error: std::fmt::Debug + From<std::io::Error>;

/// Creates a new instance.
fn new(reader: RS) -> Result<Self, Self::Error>;

}