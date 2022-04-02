use std::fmt;
use std::fmt::{Display, Formatter};

use colored::Color::{self, *};
use colored::Colorize;

const ND_PREFIX: &'static str = "[ ]";
const DD_PREFIX: &'static str = "[X]";
const AD_PREFIX: &'static str = "[~]";
const NH_PREFIX: &'static str = "[?]";

#[derive(Debug, Copy, PartialEq, Eq, Hash, Clone)]
pub enum TaskKind {
    NotDone,
    Done,
    AlmostDone,
    NeedsHelp,
}

impl TaskKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            TaskKind::NotDone => ND_PREFIX,
            TaskKind::Done => DD_PREFIX,
            TaskKind::AlmostDone => AD_PREFIX,
            TaskKind::NeedsHelp => NH_PREFIX,
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            TaskKind::NotDone => Red,
            TaskKind::Done => Green,
            TaskKind::AlmostDone => Yellow,
            TaskKind::NeedsHelp => Blue,
        }
    }

    pub fn try_from(s: &str) -> Option<TaskKind> {
        match s {
            ND_PREFIX => Some(TaskKind::NotDone),
            DD_PREFIX => Some(TaskKind::Done),
            AD_PREFIX => Some(TaskKind::AlmostDone),
            NH_PREFIX => Some(TaskKind::NeedsHelp),
            _ => None
        }
    }
}

pub struct Task {
    pub text: String,
    pub kind: TaskKind,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{} - {}", self.kind.to_str(), self.text).color(self.kind.to_color())
        )
    }
}


// TODO: write tests for this
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_kind_try_from() {
        assert_eq!(TaskKind::try_from("[ ]"), Some(TaskKind::NotDone));
        assert_eq!(TaskKind::try_from("[X]"), Some(TaskKind::Done));
        assert_eq!(TaskKind::try_from("[~]"), Some(TaskKind::AlmostDone));
        assert_eq!(TaskKind::try_from("[?]"), Some(TaskKind::NeedsHelp));
        assert_eq!(TaskKind::try_from(" [?] "), None);
        assert_eq!(TaskKind::try_from("[A]"), None);
        assert_eq!(TaskKind::try_from("Test"), None);
        assert_eq!(TaskKind::try_from("[[x]]"), None);
        assert_eq!(TaskKind::try_from("[\\]"), None);
        assert_eq!(TaskKind::try_from("[:]"), None);
    }
}