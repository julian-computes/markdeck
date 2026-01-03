mod app;
mod commands;
mod config;

use std::io::Stdout;

use anyhow::Result;
use app::{App, load_slides, node_to_lines};
use clap::Parser;
use ratatui::{
    Terminal,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyModifiers},
    },
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    text::Text,
    widgets::{Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

#[derive(Parser)]
#[command(name = "markdeck")]
#[command(about = "A terminal-based markdown presentation viewer", long_about = None)]
struct Cli {
    #[arg(help = "Path to the markdown file to present")]
    file: String,

    #[arg(short, long, help = "Path to config file (defaults to ~/.config/markdeck/config.toml)")]
    config: Option<String>,
}

pub fn render(app: &mut App, frame: &mut ratatui::Frame, config: &config::Config) {
    let area = frame.area();

    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ]);
    let [header_area, content_area, footer_area] = vertical.areas(area);

    let slide_indicator = format!("{}/{}", app.current_slide + 1, app.slides.len());
    let header = Paragraph::new(slide_indicator)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Right);
    frame.render_widget(header, header_area);

    let padded_area = content_area.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    app.viewport_height = padded_area.height;

    if let Some(slide) = app.slides.get(app.current_slide) {
        let mut all_lines = vec![];
        for node in slide {
            let mut node_lines = vec![];
            node_to_lines(node, &mut node_lines, Style::default());
            all_lines.extend(node_lines);
        }

        let num_lines = all_lines.len() as u16;
        let content_width = padded_area.width;

        let mut scroll_view = ScrollView::new((content_width, num_lines).into())
            .horizontal_scrollbar_visibility(ScrollbarVisibility::Never);

        let text = Text::from(all_lines);
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: false });

        scroll_view.render_widget(paragraph, Rect::new(0, 0, content_width, num_lines));
        frame.render_stateful_widget(scroll_view, padded_area, &mut app.scroll_view_state);
    }

    let controls_text = config.format_help_text();
    let footer = Paragraph::new(controls_text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, footer_area);
}

pub fn handle_key(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers, config: &config::Config) {
    if let Some(cmd) = config.get_command(key_code, modifiers) {
        cmd.execute(app);
    }
}

pub fn run_app(term: &mut Terminal<CrosstermBackend<Stdout>>, file_path: &str, config: config::Config) -> Result<()> {
    let slides = load_slides(file_path)?;
    let mut app = App::new(slides);

    loop {
        term.draw(|f| render(&mut app, f, &config))?;
        let event = crossterm::event::read()?;
        if let Event::Key(key) = event
            && key.is_press()
        {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
            handle_key(&mut app, key.code, key.modifiers, &config);
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::Config::load(cli.config.as_deref())?;
    ratatui::run(|term| run_app(term, &cli.file, config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j_maps_to_scroll_down() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE, &config);
    }

    #[test]
    fn test_k_maps_to_scroll_up() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('k'), KeyModifiers::NONE, &config);
    }

    #[test]
    fn test_down_arrow_maps_to_scroll_down() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Down, KeyModifiers::NONE, &config);
    }

    #[test]
    fn test_up_arrow_maps_to_scroll_up() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Up, KeyModifiers::NONE, &config);
    }

    #[test]
    fn test_h_maps_to_previous_slide() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![], vec![]]);
        app.current_slide = 1;
        handle_key(&mut app, KeyCode::Char('h'), KeyModifiers::NONE, &config);
        assert_eq!(app.current_slide, 0);
    }

    #[test]
    fn test_l_maps_to_next_slide() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![], vec![]]);
        handle_key(&mut app, KeyCode::Char('l'), KeyModifiers::NONE, &config);
        assert_eq!(app.current_slide, 1);
    }

    #[test]
    fn test_ctrl_f_maps_to_page_down() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('f'), KeyModifiers::CONTROL, &config);
    }

    #[test]
    fn test_ctrl_b_maps_to_page_up() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('b'), KeyModifiers::CONTROL, &config);
    }

    #[test]
    fn test_ctrl_d_maps_to_half_page_down() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('d'), KeyModifiers::CONTROL, &config);
    }

    #[test]
    fn test_ctrl_u_maps_to_half_page_up() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('u'), KeyModifiers::CONTROL, &config);
    }

    #[test]
    fn test_g_maps_to_jump_to_top() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('g'), KeyModifiers::NONE, &config);
    }

    #[test]
    fn test_shift_g_maps_to_jump_to_bottom() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![]]);
        handle_key(&mut app, KeyCode::Char('G'), KeyModifiers::SHIFT, &config);
    }

    #[test]
    fn test_unrecognized_key_does_nothing() {
        let config = config::Config::default();
        let mut app = App::new(vec![vec![], vec![]]);
        let initial_slide = app.current_slide;
        handle_key(&mut app, KeyCode::Char('x'), KeyModifiers::NONE, &config);
        assert_eq!(app.current_slide, initial_slide);
    }
}
