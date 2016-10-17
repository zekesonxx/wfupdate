#![allow(missing_docs)]
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::path::PathBuf;

pub fn update_file(path: PathBuf, newcontent: Vec<u8>) -> io::Result<()> {
    let f = try!(File::create(path));
    {
        let mut writer = BufWriter::new(f);
        try!(writer.write(newcontent.as_slice()));
    }
    Ok(())
}
