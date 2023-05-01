use std::io::{Read, Seek};
use quick_xml::events::{Event};
use quick_xml::name::QName;
use super::errors::*;
use super::workbook::*;
use super::reader::*;
use log::{info};




impl<RS: Read + Seek> PivotTable<RS> {


    pub fn read_pivot_cache(&mut self) -> Result<(), XlsxError> {

        let mut xml = match xml_reader(&mut self.zip, "xl/pivotCache/pivotCacheRecords1.xml") {
            None => return Ok(()),
            Some(x) => x?,
        };
        info!("Enter read_pivot_cache");

        let mut buf = Vec::new();
        loop {
            buf.clear();
            match xml.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"pivotCacheRecords" => {
                     info!("Trying to get cache data");
                     let row = get_pivot_row_data(&mut xml, &self.cache_definitions);
                     self.row_data = row?;
                     return Ok(())
                },
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
        let mut pivot_cache = Vec::<PivotCacheDefinition>::new();
        info!("Enter read_pivot_cache_definition");
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
                    pivot_cache.push(PivotCacheDefinition { cacheDefintionName:cache_name, cacheDefintionString: cached_values.unwrap() });
                }
                Ok(Event::End(ref e)) if e.local_name().as_ref() == b"cacheFields" => break,
                Ok(Event::Eof) => return Err(XlsxError::XmlEof("cacheFields")),
                Err(e) => return Err(XlsxError::Xml(e)),
                _ => (),
            }
        }
        self.cache_definitions = pivot_cache;
        info!("Got pivot cache");
        Ok(())
    }


}


fn get_pivot_row_data(
    xml: &mut XlsReader<'_>,
    pt: &Vec<PivotCacheDefinition>
) -> Result<Vec<Vec<String>>, XlsxError> {
    let mut buf = Vec::new();

    let mut column_based: Vec<Vec<String>>= Vec::new();

    let mut pivot_row_cnt = 0;
    let mut loop_cnt = 0;

    loop {
        buf.clear();
        match xml.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"r" => {
                let row_data = get_row_field_data(xml, pt, &mut pivot_row_cnt)?;
                
                if loop_cnt ==0 {
                    info!("got first row");
                    for _ in 0..row_data.len() {
                        column_based.push(Vec::<String>::new());
                    }
                }
                
                if loop_cnt % 10000 == 0 {
                    info!("{}", loop_cnt);
                }

                row_data.iter().enumerate().for_each(|(i,v)| {
                    column_based[i].push(v.to_string())
                });

                loop_cnt += 1;
                
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"pivotCacheRecords" => {
                return Ok(column_based);
            }
            Err(e) => return Err(XlsxError::Xml(e)),
            _ => (),
        }
    }
}

fn get_row_field_data(
    xml: &mut XlsReader<'_>,
    pt: &Vec<PivotCacheDefinition>,
    pivot_row_cnt: &mut i32,
) -> Result<Vec<String>, XlsxError> {
    let mut buf = Vec::new();
    let mut val_buf: Vec<u8> = Vec::new();
    let mut all_values: Vec<String> = Vec::new();
    let mut x_cnt = 0;
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
                        let ref str = pt[x_cnt].cacheDefintionString[index];
                        all_values.push(str.to_string());
                        
                    } 
                    x_cnt+= 1;
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
            Ok(Event::Start(ref e)) if e.local_name().as_ref() == b"m" => {
                val_buf.clear();
                all_values.push("".to_string());
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"r" => {
                info!("{} = {}", pivot_row_cnt, all_values.len());
                *pivot_row_cnt += 1;
                if all_values.len() != 7 {
                    info!("issue on row {} only have {} fields", pivot_row_cnt, all_values.len());
                }
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