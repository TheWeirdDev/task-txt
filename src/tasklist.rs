use crate::task::Task;
use tui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct TaskList {
    pub list_state: ListState,
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            list_state: ListState::default(),
            tasks,
        }
    }

    pub fn next(&mut self) {
        self.list_state.select(match self.list_state.selected() {
            Some(i) => Some((i.saturating_add(1)).min(self.tasks.len().saturating_sub(1))),
            None => Some(0),
        });
    }

    pub fn previous(&mut self) {
        self.list_state.select(Some(
            self.list_state.selected().unwrap_or(0).saturating_sub(1),
        ));
    }
}
