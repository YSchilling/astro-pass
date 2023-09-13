use ratatui::widgets::ListState;

use crate::database::Password;

pub struct StatefulList {
    pub state: ListState,
}

impl StatefulList {
    pub fn new() -> StatefulList {
        StatefulList {
            state: ListState::default(),
        }
    }

    pub fn next(&mut self, items: &Vec<Password>) {
        let i = match self.state.selected() {
            None => 0,
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, items: &Vec<Password>) {
        let i = match self.state.selected() {
            None => 0,
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
