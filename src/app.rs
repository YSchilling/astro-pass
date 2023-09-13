use std::{error::Error, io::Stdout};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{BorderType, Paragraph};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::{
    database::{Database, Password},
    widgets::StatefulList,
};

pub struct App {
    db: Database,
    stateful_list: StatefulList,
}

impl App {
    pub fn new() -> Self {
        let db = Database::load_from_file();
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
                        KeyCode::Char('c') => self.create_password(),
                        KeyCode::Char('d') => self.delete_password(),
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
            // Main Layout
            let main_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(f.size());

            // Menu
            let menu_block = Block::default().title("Menu").borders(Borders::ALL);
            f.render_widget(menu_block, main_layout[0]);

            let menu_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(vec![
                    Constraint::Min(0),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(main_layout[0]);

            // Username Field
            let username_block = Block::default()
                .title("Username")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let username_paragraph = Paragraph::new("").block(username_block.clone());
            f.render_widget(username_paragraph, menu_layout[1]);

            // Password Field
            let password_block = Block::default()
                .title("Password")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            let password_paragraph = Paragraph::new("").block(password_block);
            f.render_widget(password_paragraph, menu_layout[2]);

            // Password List
            let list_items: Vec<ListItem> = self
                .db
                .passwords
                .iter()
                .map(|pw| ListItem::new(format!("{pw:?}")))
                .collect();
            let list_block = Block::default().title("Passwords").borders(Borders::ALL);
            let password_list = List::new(list_items)
                .block(list_block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            f.render_stateful_widget(password_list, main_layout[1], &mut self.stateful_list.state);
        })?;

        Ok(())
    }

    fn create_password(&mut self) {
        let id = self.db.id_counter;
        self.db.id_counter += 1;
        let test_password = Password::new(id, "TestName".to_string(), "TestContent".to_string());
        self.db.create_password(test_password);
    }

    fn delete_password(&mut self) {
        let selected = match self.stateful_list.state.selected() {
            None => return,
            Some(i) => i,
        };
        let password = self.db.passwords.iter().nth(selected);
        let id = match password {
            None => panic!("Selected index doesn't match database!"),
            Some(pw) => pw.id,
        };
        self.db.delete_password(id as u32);
    }
}
