use ratatui::widgets::ListState;

use crate::database::Password;

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Password>,
}

impl StatefulList {
    pub fn with_items(items: Vec<Password>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            None => 0,
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            None => 0,
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
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
