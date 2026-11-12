//! One thing that stops a figure being derived.

use std::fmt;

/// The kinds of problem, exactly as printed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Kind {
    List,
    Rules,
    Facts,
    UnknownShip,
    NoClass,
    UnmappedType,
    Missing,
    NotPreferred,
    NotOnDate,
    Conflict,
    AmbiguousDate,
    RadarUnknown,
    BadValue,
    ZeroToughness,
    Exception,
    UnknownType,
    Experience,
}

impl Kind {
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::List => "list",
            Kind::Rules => "rules",
            Kind::Facts => "facts",
            Kind::UnknownShip => "unknown-ship",
            Kind::NoClass => "no-class",
            Kind::UnmappedType => "unmapped-type",
            Kind::Missing => "missing",
            Kind::NotPreferred => "not-preferred",
            Kind::NotOnDate => "not-on-date",
            Kind::Conflict => "conflict",
            Kind::AmbiguousDate => "ambiguous-date",
            Kind::RadarUnknown => "radar-unknown",
            Kind::BadValue => "bad-value",
            Kind::ZeroToughness => "zero-toughness",
            Kind::Exception => "exception",
            Kind::UnknownType => "unknown-type",
            Kind::Experience => "experience",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `<ship key, list, rules or facts>: <kind>: <figure or fact>: <message>`
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Problem {
    pub subject: String,
    pub kind: Kind,
    pub figure: String,
    pub message: String,
}

impl Problem {
    pub fn new(
        subject: impl Into<String>,
        kind: Kind,
        figure: impl Into<String>,
        message: impl Into<String>,
    ) -> Problem {
        Problem {
            subject: subject.into(),
            kind,
            figure: figure.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {}: {}",
            self.subject, self.kind, self.figure, self.message
        )
    }
}
