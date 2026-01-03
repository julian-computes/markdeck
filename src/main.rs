use std::io::Stdout;

use anyhow::{Result, anyhow};
use markdown::{ParseOptions, mdast::Node, to_mdast};
use ratatui::{
    Terminal,
    crossterm::{
        self,
        event::{Event, KeyCode},
    },
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

pub struct App {
    slides: Vec<Vec<Node>>,
}

fn demo() -> Result<Vec<Vec<Node>>> {
    let readme = std::fs::read_to_string("README.md")?;
    let mut mdast =
        to_mdast(readme.as_str(), &ParseOptions::default()).map_err(|e| anyhow!("{}", e))?;

    let mut current_slide_content = vec![];
    let mut slides = vec![];
    let children = mdast.children_mut().ok_or(anyhow!("No children"))?;

    for node in children {
        if !current_slide_content.is_empty()
            && let Node::Heading(_) = node
        {
            // Move the current slide into the slides list
            slides.push(std::mem::take(&mut current_slide_content));
        }

        current_slide_content.push(node.clone());
    }

    // Push the last slide
    slides.push(current_slide_content);

    Ok(slides)
}

pub fn render_node(node: &Node, _frame: &mut ratatui::Frame) {
    let children = node.children().unwrap();
    dbg!(children);
}

pub fn render(app: &App, frame: &mut ratatui::Frame) {
    let area = frame.area();

    let text = "Hello world";
    let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
    let [instructions, _] = vertical.areas(area);

    let paragraph = Paragraph::new(text.slow_blink())
        .centered()
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, instructions);

    let node = app.slides.first().unwrap().first().unwrap();
    render_node(node, frame);
}

pub fn handle_key(_app: &mut App) {}

pub fn run_app(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let slides = demo()?;
    let app = App { slides };
    loop {
        term.draw(|f| render(&app, f))?;
        let event = crossterm::event::read()?;
        if let Event::Key(key) = event
            && key.is_press()
            && let KeyCode::Char('q') = key.code
        {
            return Ok(());
        }
    }
}

fn main() -> Result<()> {
    ratatui::run(run_app)
}
