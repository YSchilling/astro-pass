use std::{error::Error, io::Stdout};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::{database::Database, widgets::StatefulList};

pub struct App {
    db: Database,
    stateful_list: StatefulList,
}

impl App {
    pub fn new() -> Self {
        let db = Database::new();
        let stateful_list = StatefulList::new();

        Self { db, stateful_list }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        'app_loop: loop {
            self.draw(terminal)?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break 'app_loop,
                        KeyCode::Down => self.stateful_list.next(&self.db.passwords),
                        KeyCode::Up => self.stateful_list.previous(&self.db.passwords),
                        KeyCode::Left => self.stateful_list.unselect(),
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        terminal.draw(|f| {
            //Layout

            let block = Block::default().title("Passwords").borders(Borders::ALL);

            let items: Vec<ListItem> = self
                .db
                .passwords
                .iter()
                .map(|pw| ListItem::new(format!("{pw:?}")))
                .collect();

            let list = List::new(items)
                .block(block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            f.render_stateful_widget(list, f.size(), &mut self.stateful_list.state);
        })?;

        Ok(())
    }
}
