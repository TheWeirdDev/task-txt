mod task;

use task::TaskKind;

use crate::task::Task;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() -> Result<(), std::io::Error> {
    let lines = read_lines("tasks.txt")?;
    let tasks = lines
        .filter_map(|l| {
            let line = l.ok()?;
            let (kind, text) = line.split_once(" - ")?;
            Some(Task {
                text: text.trim().to_string(),
                kind: TaskKind::try_from(kind.trim())?,
            })
        })
        .collect::<Vec<Task>>();
    for task in tasks {
        println!("{}", task);
    }

    Ok(())
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
