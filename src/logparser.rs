//! Utility for parsing a Warframe log lines into a Rust-readable format.
//!
//! logparser is only concerned with log lines pertaining to updates, and doesn't parse anything else.
//! # Log File Format
//! The log lines we care about look something like these:
//!
//! ```text
//! 14.165 Sys [Info]: /Lotus/Levels/Proc/Orokin/OrokinMoonDefense is out of date (hash mismatch)
//! 14.182 Sys [Info]: 4,493,854,909 bytes to download
//! 16.217 Sys [Info]: Used shared /Lotus/Levels/Proc/Orokin/OrokinMoonDefense (25,705B Copy: 1.85s Write: 0s Latency: 2.01s)
//! ```
//! * The first one, `(hash mismatch)`, indicates a file that needs updating.
//! * The middle one indicates what you would think, the total size of all the files to download.
//! * And the last one indicates a finished file download.
//!
//! The lines above will *usually* occur in that order: a lot of hash mismatches, one bytes to download, and then lots of used shared.
//!
//! However, there are some lines that look like this:
//!
//! ```text
//! 1.187 Sys [Info]: Used shared /H.Cache.bin (106B Copy: 0s Write: 0.152s Latency: 0.488s)
//! ```
//!
//! These occur very early in the log, and represent the launcher downloading file lists, which it uses to do the later hash mismatches.
//! At the moment wfupdate doesn't distinugish against these lines, but it probably should.
//!
//! # Output
//! The above lines, parsed, would equal these returned LogLines:
//!
//! ```rust,ignore
//! HashMismatch("/Lotus/Levels/Proc/Orokin/OrokinMoonDefense");
//! BytesToDownload(4493854909);
//! UsedShared(25705, "/Lotus/Levels/Proc/Orokin/OrokinMoonDefense");
//! ```
#![warn(missing_docs)]
extern crate regex;

use std::fmt;
use self::regex::Regex;

/// A parsed Warframe log line.
///
/// 64-bit unsigned integers are used instead of usize to ensure it doesn't run into problems on 32-bit systems.
#[derive(Debug, Eq, PartialEq)]
pub enum LogLine {
    /// The name of the file that had a mismatched hash
    HashMismatch(String),
    /// The amount of bytes to download
    BytesToDownload(u64),
    /// The size and name of the file downloaded
    UsedShared(u64, String),
    /// Unknown line, the included string is the original line.
    Unknown(String)
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            LogLine::HashMismatch(_) => "HashMismatch",
            LogLine::BytesToDownload(_) => "BytesToDownload",
            LogLine::UsedShared(_, _) => "UsedShared",
            LogLine::Unknown(_) => "Unknown"
        })
    }
}


/// Parse a comma-delimited string into a u64
///
/// # Example
/// ```
/// assert_eq!(parse_bytes("1,337,420"), 1337420);
/// ```
fn parse_bytes(input: &str) -> u64 {
    if input.len() == 0 {
        return 0;
    }
    let mut string = String::from(input);
    let mut i: usize = 0;
    for c in input.chars() {
        if c == ',' {
            string.remove(i);
        } else {
            i += 1;
        }
    }
    string.parse::<>().unwrap()
}


/// Parses a line and returns a LogLine representing the usable value of the line.
///
/// See the documentation for the crate for more info.
///
/// # Arguments
/// * `line` The line to parse
///
pub fn parse_line(line: &str) -> LogLine {
    lazy_static! {
        static ref RE_USEDSHARED: Regex =
            Regex::new(r"[0-9\.]+\sSys\s\[Info\]: Used shared (?P<file>[^\s]+) \((?P<size>[0-9,]+)[^)]+\)").unwrap();
        static ref RE_BYTESTODOWNLOAD: Regex =
            Regex::new(r"[0-9\.]+\sSys\s\[Info\]: (?P<size>[0-9,]+) bytes to download").unwrap();
        static ref RE_HASHMISMATCH: Regex =
            Regex::new(r"[0-9\.]+\sSys\s\[Info\]: (?P<file>[^\s]+) is out of date \(hash mismatch\)").unwrap();
    }

    if let Some(captures) = RE_USEDSHARED.captures(line) {
        return LogLine::UsedShared(parse_bytes(captures.name("size").unwrap()), String::from(captures.name("file").unwrap()));
    }
    if let Some(captures) = RE_HASHMISMATCH.captures(line) {
        return LogLine::HashMismatch(String::from(captures.name("file").unwrap()));
    }
    if let Some(captures) = RE_BYTESTODOWNLOAD.captures(line) {
        return LogLine::BytesToDownload(parse_bytes(captures.name("size").unwrap()));
    }
    LogLine::Unknown(String::from(""))
}

#[cfg(test)]
mod tests {

    use super::LogLine::*;

    #[test]
    pub fn test_parse_bytes() {
        assert_eq!(super::parse_bytes(""), 0);
        assert_eq!(super::parse_bytes("4"), 4);
        assert_eq!(super::parse_bytes("14,520"), 14520);
        assert_eq!(super::parse_bytes("1,2,3,4,5"), 12345);
        assert_eq!(super::parse_bytes("4,493,854,909"), 4493854909);
    }

    #[test]
    pub fn test_shouldnt_fail_on_empty_input() {
        super::parse_line("");
    }

    #[test]
    pub fn test_detect_hashmismatch() {
        let result = super::parse_line("4.489 Sys [Info]: /Lotus/Levels/OrokinDerelict/PipeConnectorDerelict1/0_c.fbx is out of date (hash mismatch)");
        assert_eq!(result, HashMismatch(String::from("/Lotus/Levels/OrokinDerelict/PipeConnectorDerelict1/0_c.fbx")));
    }

    #[test]
    pub fn test_detect_bytestodownload() {
        let result = super::parse_line("14.182 Sys [Info]: 4,493,854,909 bytes to download");
        assert_eq!(result, BytesToDownload(4493854909));
    }

    #[test]
    pub fn test_detect_usedshared() {
        let result = super::parse_line("109.880 Sys [Info]: Used shared /Lotus/Objects/Natural/Skybox/TennoHanger/GasPlaneOptA.fbx (3,607B Copy: 1.20s Write: 0s Latency: 95.6s)");
        assert_eq!(result, UsedShared(3607, String::from("/Lotus/Objects/Natural/Skybox/TennoHanger/GasPlaneOptA.fbx")));
    }
}
