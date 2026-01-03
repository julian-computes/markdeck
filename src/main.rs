mod app;
mod commands;

use std::io::Stdout;

use anyhow::Result;
use app::{App, load_slides, node_to_lines};
use commands::Command;
use ratatui::{
    Terminal,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyModifiers},
    },
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::Style,
    text::Text,
    widgets::{Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

pub fn render(app: &mut App, frame: &mut ratatui::Frame) {
    let area = frame.area();

    let vertical = Layout::vertical([Constraint::Percentage(100)]);
    let [content_area] = vertical.areas(area);

    app.viewport_height = content_area.height;

    if let Some(slide) = app.slides.get(app.current_slide) {
        let mut all_lines = vec![];
        for node in slide {
            let mut node_lines = vec![];
            node_to_lines(node, &mut node_lines, Style::default());
            all_lines.extend(node_lines);
        }

        let num_lines = all_lines.len() as u16;
        let content_width = content_area.width;

        let mut scroll_view = ScrollView::new((content_width, num_lines).into())
            .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

        let text = Text::from(all_lines);
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });

        scroll_view.render_widget(paragraph, Rect::new(0, 0, content_width, num_lines));
        frame.render_stateful_widget(scroll_view, content_area, &mut app.scroll_view_state);
    }
}

pub fn handle_key(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) {
    let command = match (key_code, modifiers) {
        // Single line scrolling
        (KeyCode::Char('j'), KeyModifiers::NONE) => Some(Command::ScrollDown),
        (KeyCode::Char('k'), KeyModifiers::NONE) => Some(Command::ScrollUp),
        // Slide navigation
        (KeyCode::Char('h'), KeyModifiers::NONE) => Some(Command::PreviousSlide),
        (KeyCode::Char('l'), KeyModifiers::NONE) => Some(Command::NextSlide),
        // Page scrolling
        (KeyCode::Char('f'), KeyModifiers::CONTROL) => Some(Command::PageDown),
        (KeyCode::Char('b'), KeyModifiers::CONTROL) => Some(Command::PageUp),
        // Half-page scrolling
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Command::HalfPageDown),
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Some(Command::HalfPageUp),
        // Jump to top/bottom
        (KeyCode::Char('g'), KeyModifiers::NONE) => Some(Command::JumpToTop),
        (KeyCode::Char('G'), KeyModifiers::NONE | KeyModifiers::SHIFT) => Some(Command::JumpToBottom),
        _ => None,
    };

    if let Some(cmd) = command {
        cmd.execute(app);
    }
}

pub fn run_app(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let slides = load_slides("README.md")?;
    let mut app = App::new(slides);

    loop {
        term.draw(|f| render(&mut app, f))?;
        let event = crossterm::event::read()?;
        if let Event::Key(key) = event
            && key.is_press()
        {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            handle_key(&mut app, key.code, key.modifiers);
        }
    }
}

fn main() -> Result<()> {
    ratatui::run(run_app)
}
