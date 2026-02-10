use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Linter {
    Air,
    Era,
    Fast,
    Ytt,
    Ann,
    Async,
    S,
    Ble,
    Fbt,
    B,
    A,
    Com,
    C4,
    Cpy,
    Dtz,
    T10,
    Dj,
    Em,
    Exe,
    Fix,
    Fa,
    Int,
    Isc,
    Icn,
    Log,
    G,
    Inp,
    Pie,
    T20,
    Pyi,
    Pt,
    Q,
    Rse,
    Ret,
    Slf,
    Sim,
    Slot,
    Tid,
    Td,
    Tc,
    Arg,
    Pth,
    Fly,
    I,
    C90,
    Npy,
    Pd,
    N,
    Perf,
    E,
    W,
    Doc,
    D,
    F,
    Pgh,
    Plc,
    Ple,
    Plr,
    Plw,
    Up,
    Furb,
    Ruf,
    Try,
}

impl fmt::Display for Linter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let caps = format!("{:?}", self).to_uppercase();
        write!(f, "{}", caps)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseLinterError;

impl FromStr for Linter {
    type Err = ParseLinterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut linter_vector: Vec<Linter> = Linter::iter().collect();
        linter_vector.sort_by_key(|item| std::cmp::Reverse(format!("{}", item).len()));

        for linter in linter_vector {
            let name = format!("{}", linter);
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
            Linter::Air => "airflow",
            Linter::Era => "eradicate",
            Linter::Fast => "fastapi",
            Linter::Ytt => "flake8-2020",
            Linter::Ann => "flake8-annotations",
            Linter::Async => "flake8-async",
            Linter::S => "flake8-bandit",
            Linter::Ble => "flake8-blind-except",
            Linter::Fbt => "flake8-boolean-trap",
            Linter::B => "flake8-bugbear",
            Linter::A => "flake8-builtins",
            Linter::Com => "flake8-commas",
            Linter::C4 => "flake8-comprehensions",
            Linter::Cpy => "flake8-copyright",
            Linter::Dtz => "flake8-datetimez",
            Linter::T10 => "flake8-debugger",
            Linter::Dj => "flake8-django",
            Linter::Em => "flake8-errmsg",
            Linter::Exe => "flake8-executable",
            Linter::Fix => "flake8-fixme",
            Linter::Fa => "flake8-future-annotations",
            Linter::Int => "flake8-gettext",
            Linter::Isc => "flake8-implicit-str-concat",
            Linter::Icn => "flake8-import-conventions",
            Linter::Log => "flake8-logging",
            Linter::G => "flake8-logging-format",
            Linter::Inp => "flake8-no-pep420",
            Linter::Pie => "flake8-pie",
            Linter::T20 => "flake8-print",
            Linter::Pyi => "flake8-pyi",
            Linter::Pt => "flake8-pytest-style",
            Linter::Q => "flake8-quotes",
            Linter::Rse => "flake8-raise",
            Linter::Ret => "flake8-return",
            Linter::Slf => "flake8-self",
            Linter::Sim => "flake8-simplify",
            Linter::Slot => "flake8-slots",
            Linter::Tid => "flake8-tidy-imports",
            Linter::Td => "flake8-todos",
            Linter::Tc => "flake8-type-checking",
            Linter::Arg => "flake8-unused-arguments",
            Linter::Pth => "flake8-use-pathlib",
            Linter::Fly => "flynt",
            Linter::I => "isort",
            Linter::C90 => "mccabe",
            Linter::Npy => "numpy-specific-rules",
            Linter::Pd => "pandas-vet",
            Linter::N => "pep8-naming",
            Linter::Perf => "perflint",
            Linter::E => "error",
            Linter::W => "warning",
            Linter::Doc => "pydoclint",
            Linter::D => "pydocstyle",
            Linter::F => "pyflakes",
            Linter::Pgh => "pygrep-hooks",
            Linter::Plc => "convention",
            Linter::Ple => "error",
            Linter::Plr => "refactor",
            Linter::Plw => "warning",
            Linter::Up => "pyupgrade",
            Linter::Furb => "refurb",
            Linter::Ruf => "ruff-specific-rules",
            Linter::Try => "tryceratops",
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
