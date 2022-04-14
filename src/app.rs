use tui::widgets::ListState;

use crate::{
    task::{read_tasks, Task, TaskKind},
    tasklist::TaskList,
    utils::TabsState,
};

pub struct App<'a> {
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub title: &'a str,
    pub done_tasks: TaskList,
    pub not_done_tasks: TaskList,
    pub almost_done_tasks: TaskList,
    pub unsure_tasks: TaskList,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, std::io::Error> {
        let all_tasks = read_tasks()?;
        let filter_tasks = |kind| {
            all_tasks
                .iter()
                .filter(|t| t.kind == kind)
                .cloned()
                .collect()
        };

        Ok(Self {
            should_quit: false,
            tabs: TabsState::new(vec!["Not Done", "Almost Done", "Done", "Unsure"]),
            title: "Task txt",
            not_done_tasks: TaskList::new(filter_tasks(TaskKind::NotDone)),
            done_tasks: TaskList::new(filter_tasks(TaskKind::Done)),
            almost_done_tasks: TaskList::new(filter_tasks(TaskKind::AlmostDone)),
            unsure_tasks: TaskList::new(filter_tasks(TaskKind::Unsure)),
        })
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_up(&mut self) {
        match self.tabs.index {
            0 => self.not_done_tasks.previous(),
            1 => self.almost_done_tasks.previous(),
            2 => self.done_tasks.previous(),
            3 => self.unsure_tasks.previous(),
            _ => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.tabs.index {
            0 => self.not_done_tasks.next(),
            1 => self.almost_done_tasks.next(),
            2 => self.done_tasks.next(),
            3 => self.unsure_tasks.next(),
            _ => {}
        }
    }
}
