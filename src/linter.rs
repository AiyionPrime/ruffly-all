use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Linter {
    AIR,
    ERA,
    FAST,
    YTT,
    ANN,
    ASYNC,
    S,
    BLE,
    FBT,
    B,
    A,
    COM,
    C4,
    CPY,
    DTZ,
    T10,
    DJ,
    EM,
    EXE,
    FIX,
    FA,
    INT,
    ISC,
    ICN,
    LOG,
    G,
    INP,
    PIE,
    T20,
    PYI,
    PT,
    Q,
    RSE,
    RET,
    SLF,
    SIM,
    SLOT,
    TID,
    TD,
    TC,
    ARG,
    PTH,
    FLY,
    I,
    C90,
    NPY,
    PD,
    N,
    PERF,
    E,
    W,
    DOC,
    D,
    F,
    PGH,
    PLC,
    PLE,
    PLR,
    PLW,
    UP,
    FURB,
    RUF,
    TRY,
}

impl fmt::Display for Linter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLinterError;

impl FromStr for Linter {
    type Err = ParseLinterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut linter_vector: Vec<Linter> = Linter::iter().collect();
        linter_vector.sort_by_key(|item| std::cmp::Reverse(format!("{:?}", item).len()));

        for linter in linter_vector {
            let name = format!("{:?}", linter);
            if s.starts_with(&name) {
                return Ok(linter);
            }
        }
        Err(ParseLinterError)
    }
}

impl Linter {
    pub fn name(&self) -> &str {
        match *self {
            Linter::AIR => "airflow",
            Linter::ERA => "eradicate",
            Linter::FAST => "fastapi",
            Linter::YTT => "flake8-2020",
            Linter::ANN => "flake8-annotations",
            Linter::ASYNC => "flake8-async",
            Linter::S => "flake8-bandit",
            Linter::BLE => "flake8-blind-except",
            Linter::FBT => "flake8-boolean-trap",
            Linter::B => "flake8-bugbear",
            Linter::A => "flake8-builtins",
            Linter::COM => "flake8-commans",
            Linter::C4 => "flake8-comprehensions",
            Linter::CPY => "flake8-copyright",
            Linter::DTZ => "flake8-datetimez",
            Linter::T10 => "flake8-debugger",
            Linter::DJ => "flake8-django",
            Linter::EM => "flake8-errmsg",
            Linter::EXE => "flake8-executable",
            Linter::FIX => "flake8-fixme",
            Linter::FA => "flake8-future-annotations",
            Linter::INT => "flake8-gettext",
            Linter::ISC => "flake8-implicit-str-concat",
            Linter::ICN => "flake8-import-conventions",
            Linter::LOG => "flake8-logging",
            Linter::G => "flake8-logging-format",
            Linter::INP => "flake8-no-pep420",
            Linter::PIE => "flake8-pie",
            Linter::T20 => "flake8-print",
            Linter::PYI => "flake8-pyi",
            Linter::PT => "flake8-pytest-style",
            Linter::Q => "flake8-quotes",
            Linter::RSE => "flake8-raise",
            Linter::RET => "flake8-return",
            Linter::SLF => "flake8-self",
            Linter::SIM => "flake8-simplify",
            Linter::SLOT => "flake8-slots",
            Linter::TID => "flake8-tidy-imports",
            Linter::TD => "flake8-toos",
            Linter::TC => "flake8-type-checking",
            Linter::ARG => "flake8-unused-arguments",
            Linter::PTH => "flake8-use-pathlib",
            Linter::FLY => "flynt",
            Linter::I => "isort",
            Linter::C90 => "mccabe",
            Linter::NPY => "numpy-pecific-rules",
            Linter::PD => "pandas-vet",
            Linter::N => "pep8-naming",
            Linter::PERF => "perflint",
            Linter::E => "error",
            Linter::W => "warning",
            Linter::DOC => "pydoclint",
            Linter::D => "pydocstyle",
            Linter::F => "pyflakes",
            Linter::PGH => "pygrep-hooks",
            Linter::PLC => "convention",
            Linter::PLE => "error",
            Linter::PLR => "refactor",
            Linter::PLW => "warning",
            Linter::UP => "pyupgrade",
            Linter::FURB => "refurb",
            Linter::RUF => "ruff-specific-rules",
            Linter::TRY => "tryceratops",
        }
    }

    pub fn generate_rule_url(&self) -> String {
        format!(
            "https://docs.astral.sh/ruff/rules/#{}-{}",
            self.name(),
            self
        )
        .to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t100() {
        let str_code = "T100";
        let code = Linter::from_str(&str_code).unwrap();
        assert_eq!(code.to_string(), "T10");
    }

    #[test]
    fn t203() {
        let str_code = "T203";
        let code = Linter::from_str(&str_code).unwrap();
        assert_eq!(code.to_string(), "T20");
    }

    #[test]
    fn q100() {
        let str_code = "Q100";
        let code = Linter::from_str(&str_code).unwrap();
        assert_eq!(code.to_string(), "Q");
    }

    #[test]
    fn plw0108() {
        let str_code = "PLW0108";
        let code = Linter::from_str(&str_code).unwrap();
        assert_eq!(code.to_string(), "PLW");
    }

    #[test]
    #[should_panic = "ParseLinterError"]
    fn invalid() {
        Linter::from_str("invalid").unwrap();
    }

    #[test]
    #[should_panic = "ParseLinterError"]
    fn invalid_numeric() {
        Linter::from_str("42").unwrap();
    }
}
