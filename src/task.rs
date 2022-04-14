use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use tui::style::Color;

const ND_PREFIX: &str = "[ ]";
const DD_PREFIX: &str = "[X]";
const AD_PREFIX: &str = "[~]";
const NH_PREFIX: &str = "[?]";

#[derive(Debug, Copy, PartialEq, Eq, Hash, Clone)]
pub enum TaskKind {
    NotDone,
    Done,
    AlmostDone,
    Unsure,
}

impl TaskKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            TaskKind::NotDone => ND_PREFIX,
            TaskKind::Done => DD_PREFIX,
            TaskKind::AlmostDone => AD_PREFIX,
            TaskKind::Unsure => NH_PREFIX,
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            TaskKind::NotDone => Color::Red,
            TaskKind::Done => Color::Green,
            TaskKind::AlmostDone => Color::Yellow,
            TaskKind::Unsure => Color::Blue,
        }
    }

    pub fn try_from(s: &str) -> Option<TaskKind> {
        match s {
            ND_PREFIX => Some(TaskKind::NotDone),
            DD_PREFIX => Some(TaskKind::Done),
            AD_PREFIX => Some(TaskKind::AlmostDone),
            NH_PREFIX => Some(TaskKind::Unsure),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Task {
    pub text: String,
    pub kind: TaskKind,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.kind.to_str(), self.text)
    }
}

pub fn read_tasks() -> Result<Vec<Task>, std::io::Error> {
    let lines = read_lines("tasks.txt")?;
    let tasks: Vec<Task> = lines
        .filter_map(|l| {
            let line = l.ok()?;
            let (kind, text) = line.split_once(" - ")?;
            Some(Task {
                text: text.trim().to_string(),
                kind: TaskKind::try_from(kind.trim())?,
            })
        })
        .collect();
    Ok(tasks)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
        assert_eq!(TaskKind::try_from("[?]"), Some(TaskKind::Unsure));
        assert_eq!(TaskKind::try_from(" [?] "), None);
        assert_eq!(TaskKind::try_from("[A]"), None);
        assert_eq!(TaskKind::try_from("Test"), None);
        assert_eq!(TaskKind::try_from("[[x]]"), None);
        assert_eq!(TaskKind::try_from("[\\]"), None);
        assert_eq!(TaskKind::try_from("[:]"), None);
    }
}
