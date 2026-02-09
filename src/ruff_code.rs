use crate::linter::Linter;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct RuffCode {
    pub linter: Linter,
    code: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRuffCodeError;

impl fmt::Display for RuffCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl FromStr for RuffCode {
    type Err = ParseRuffCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let linter = Linter::from_str(s).map_err(|_| ParseRuffCodeError)?;
        let linter_name = linter.to_string();
        let remainder = s.strip_prefix(&linter_name).ok_or(ParseRuffCodeError)?;
        let _number = remainder.parse::<u16>().map_err(|_| ParseRuffCodeError)?;
        Ok(RuffCode {
            linter,
            code: s.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c404() {
        let str_code = "C404";
        let code = RuffCode::from_str(&str_code).unwrap();
        assert_eq!(code.linter, Linter::C4);
        assert_eq!(code.to_string(), "C404");
    }

    #[test]
    fn i001() {
        let str_code = "I001";
        let code = RuffCode::from_str(&str_code).unwrap();
        assert_eq!(code.linter, Linter::I);
        assert_eq!(code.to_string(), "I001");
    }

    #[test]
    fn perf403() {
        let str_code = "PERF403";
        let code = RuffCode::from_str(&str_code).unwrap();
        assert_eq!(code.linter, Linter::PERF);
        assert_eq!(code.to_string(), "PERF403");
    }

    #[test]
    fn t203() {
        let str_code = "T203";
        let code = RuffCode::from_str(&str_code).unwrap();
        assert_eq!(code.linter, Linter::T20);
        assert_eq!(code.to_string(), "T203");
    }

    #[test]
    #[should_panic = "ParseRuffCodeError"]
    fn invalid() {
        RuffCode::from_str("invalid").unwrap();
    }

    #[test]
    #[should_panic = "ParseRuffCodeError"]
    fn invalid_numeric() {
        RuffCode::from_str("42").unwrap();
    }
}
