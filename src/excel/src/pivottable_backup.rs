use std::borrow::{Cow, Borrow};
use std::io::{Read, Seek};
use quick_xml::events::attributes::{Attribute, Attributes};
use quick_xml::events::{Event};
use quick_xml::name::QName;
use super::errors::*;
use super::workbook::*;
use super::reader::*;




impl<RS: Read + Seek> PivotTable<RS> {


    pub fn read_pivot_cache(&mut self) -> Result<(), XlsxError> {

        let mut xml = match xml_reader(&mut self.zip, "xl/pivotCache/pivotCacheRecords1.xml") {
            None => return Ok(()),
            Some(x) => x?,
        };

        let mut data = Vec::new();

        let mut buf = Vec::new();
        loop {
            buf.clear();
            match xml.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"pivotCacheRecords" => {
                     let row = get_pivot_row_data(&mut xml, &self.cache_definitions);
                     data = row?;
                     self.row_data = data;
                     return Ok(())
                },
                /* 
                Ok(Event::End(ref e)) if e.local_name().as_ref() == b"pivotCacheRecords" => {
                    println!("data is {}", data.len());
                    self.row_data = data;
                    return Ok(());
                },
                */
                Err(e) => return Err(XlsxError::Xml(e)),
                _ => (),
            }
        }


    }

    pub fn read_pivot_cache_defintion(&mut self) -> Result<(), XlsxError> {
        let mut xml = match xml_reader(&mut self.zip, "xl/pivotCache/pivotCacheDefinition1.xml") {
            None => return Ok(()),
            Some(x) => x?,
        };
        let mut buf = Vec::new();
        let mut pivotCache = Vec::<PivotCacheDefinition>::new();
        
        loop {
            buf.clear();
            match xml.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"cacheField" => {
                    let mut cache_name = String::new();
                    let xml_reader = &mut xml;
                    e.attributes().for_each(|a| {
                        let att = a.unwrap();
                        if att.key == QName("name".as_bytes()) {
                            cache_name.push_str(&att.decode_and_unescape_value(xml_reader).unwrap());
                        }
                    });
                    let cached_values =  read_pivot_cache_shared_items(xml_reader);
                    pivotCache.push(PivotCacheDefinition { cacheDefintionName:cache_name, cacheDefintionString: cached_values.unwrap() });
                }
                Ok(Event::End(ref e)) if e.local_name().as_ref() == b"cacheFields" => break,
                Ok(Event::Eof) => return Err(XlsxError::XmlEof("cacheFields")),
                Err(e) => return Err(XlsxError::Xml(e)),
                _ => (),
            }
        }
        self.cache_definitions = pivotCache;
        Ok(())
    }


}


fn get_pivot_row_data(
    xml: &mut XlsReader<'_>,
    pt: &Vec<PivotCacheDefinition>
) -> Result<Vec<Vec<String>>, XlsxError> {
    let mut buf = Vec::new();
    let mut val_buf: Vec<u8> = Vec::new();
    let mut all_values =  Vec::new();
    loop {
        buf.clear();
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"r" => {
                all_values.push(get_row_field_data(xml, pt)?);
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"pivotCacheRecords" => {
                return Ok(all_values);
            }
            Err(e) => return Err(XlsxError::Xml(e)),
            _ => (),
        }
    }
}

fn get_row_field_data(
    xml: &mut XlsReader<'_>,
    pt: &Vec<PivotCacheDefinition>,
    colData: Vec<Vec<String>>
) -> Result<Vec<String>, XlsxError> {
    let mut buf = Vec::new();
    let mut val_buf: Vec<u8> = Vec::new();
    let mut all_values: Vec<String> = Vec::new();
    let mut xCnt = 0;
    loop {
        buf.clear();
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"x" => {
                val_buf.clear();
                e.attributes().into_iter().for_each(|a|  {
                    let att = a.unwrap();
                    let mut value = String::new();
                    if att.key ==  QName("v".as_bytes()) {
                        value.push_str(&att.decode_and_unescape_value(xml).unwrap());
                        let index = value.parse::<usize>().unwrap();
                        let ref str = pt[xCnt].cacheDefintionString[index];
                        all_values.push(str.to_string());
                        
                    } 
                    xCnt+= 1;
                });
            }
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"n" => {
                val_buf.clear();
                e.attributes().into_iter().for_each(|a|  {
                    let att = a.unwrap();
                    let mut value = String::new();
                    if att.key ==  QName("v".as_bytes()) {
                        value.push_str(&att.decode_and_unescape_value(xml).unwrap());
                        all_values.push(value);
                    } 
                });
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"r" => {
                return Ok(all_values);
            }
            _ => (),
        }
    }
}

fn read_pivot_cache_shared_items(
    xml: &mut XlsReader<'_>,
) -> Result<Vec<String>, XlsxError> {
    let mut buf = Vec::new();
    let mut val_buf: Vec<u8> = Vec::new();
    let mut all_values: Vec<String> = Vec::new();
    loop {
        buf.clear();
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"s" => {
                val_buf.clear();
                e.attributes().into_iter().for_each(|a|  {
                    let att = a.unwrap();
                    let mut value = String::new();
                    value.push_str(&att.decode_and_unescape_value(xml).unwrap());
                    if att.key ==  QName("v".as_bytes()) {
                        all_values.push(value);
                    } 
                });
            }
            //TODO - FIX THIS (s and n) string and number
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"n" => {
                val_buf.clear();
                e.attributes().into_iter().for_each(|a|  {
                    let att = a.unwrap();
                    let mut value = String::new();
                    value.push_str(&att.decode_and_unescape_value(xml).unwrap());
                    if att.key ==  QName("v".as_bytes()) {
                        all_values.push(value);
                    } 
                });
            }
            //TODO - FIX THIS m is a null / blank
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"m" => {
                all_values.push("".to_string());
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"sharedItems" => {
                return Ok(all_values);
            }
            _ => (),
        }
    }
}