extern crate regex;

use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct RuffCode {
    pub group: String,
    number: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRuffCodeError;

impl fmt::Display for RuffCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:03}", self.group, self.number)
    }
}

impl FromStr for RuffCode {
    type Err = ParseRuffCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex_code: regex::Regex = Regex::new(r"(?m)([A-Z]+)([0-9]+)").unwrap();

        let matches = regex_code.captures(s).ok_or(ParseRuffCodeError)?;
        let group = matches[1].to_string();
        let number = matches[2].parse::<u16>().map_err(|_| ParseRuffCodeError)?;
        Ok(RuffCode { group, number })
    }
}
