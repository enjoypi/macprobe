use crate::log::Ctx;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Granted,
    Denied,
    Unknown,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Granted => "granted",
            Status::Denied => "denied",
            Status::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub id: String,
    pub name: String,
    pub status: Status,
    pub detail: String,
}

pub trait Probe {
    fn check(&self, ctx: &Ctx) -> Outcome;
}

pub trait Inspector {
    fn inspect(&self, ctx: &Ctx);
}

pub fn overall(outcomes: &[Outcome]) -> Status {
    let has = |s: Status| outcomes.iter().any(|o| o.status == s);
    if has(Status::Unknown) {
        Status::Unknown
    } else if has(Status::Denied) {
        Status::Denied
    } else {
        Status::Granted
    }
}

#[cfg(test)]
#[path = "probe_test.rs"]
mod tests;
