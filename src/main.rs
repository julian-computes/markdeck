use std::io::Stdout;

use anyhow::{Result, anyhow};
use markdown::{ParseOptions, mdast::Node, to_mdast};
use ratatui::{
    Terminal,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyModifiers},
    },
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

pub struct App {
    slides: Vec<Vec<Node>>,
    current_slide: usize,
    scroll_view_state: ScrollViewState,
    viewport_height: u16,
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
            && let Node::Heading(heading) = node
            && (heading.depth == 1 || heading.depth == 2)
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

fn node_to_lines(node: &Node, lines: &mut Vec<Line<'static>>, style: Style) {
    match node {
        Node::Root(root) => {
            for child in &root.children {
                node_to_lines(child, lines, style);
            }
        }
        Node::Heading(heading) => {
            let level = heading.depth;
            let heading_style = Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD);

            let prefix = "#".repeat(level as usize) + " ";
            let mut spans = vec![Span::styled(prefix, heading_style)];

            for child in &heading.children {
                collect_inline_spans(child, &mut spans, heading_style);
            }

            lines.push(Line::from(spans));
            lines.push(Line::raw(""));
        }
        Node::Paragraph(paragraph) => {
            let mut spans = vec![];
            for child in &paragraph.children {
                collect_inline_spans(child, &mut spans, style);
            }
            lines.push(Line::from(spans));
            lines.push(Line::raw(""));
        }
        Node::List(list) => {
            for (i, child) in list.children.iter().enumerate() {
                if let Node::ListItem(item) = child {
                    let bullet = if list.ordered {
                        format!("{}. ", i + 1)
                    } else {
                        "• ".to_string()
                    };

                    let mut item_spans = vec![Span::raw(bullet)];
                    for item_child in &item.children {
                        collect_inline_spans(item_child, &mut item_spans, style);
                    }
                    lines.push(Line::from(item_spans));
                }
            }
            lines.push(Line::raw(""));
        }
        Node::Code(code) => {
            let code_style = Style::default().fg(Color::Green).bg(Color::DarkGray);

            if let Some(lang) = &code.lang {
                lines.push(Line::styled(format!("```{}", lang), code_style));
            } else {
                lines.push(Line::styled("```", code_style));
            }

            for line in code.value.lines() {
                lines.push(Line::styled(line.to_string(), code_style));
            }
            lines.push(Line::styled("```", code_style));
            lines.push(Line::raw(""));
        }
        Node::Blockquote(quote) => {
            for child in &quote.children {
                let quote_style = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC);

                let mut quote_lines = vec![];
                node_to_lines(child, &mut quote_lines, quote_style);

                for line in quote_lines {
                    let mut spans = vec![Span::raw("> ")];
                    spans.extend(line.spans);
                    lines.push(Line::from(spans));
                }
            }
        }
        Node::ThematicBreak(_) => {
            lines.push(Line::raw("─".repeat(40)));
            lines.push(Line::raw(""));
        }
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    node_to_lines(child, lines, style);
                }
            }
        }
    }
}

fn collect_inline_spans(node: &Node, spans: &mut Vec<Span<'static>>, base_style: Style) {
    match node {
        Node::Text(text) => {
            spans.push(Span::styled(text.value.clone(), base_style));
        }
        Node::Strong(strong) => {
            let bold_style = base_style.add_modifier(Modifier::BOLD);
            for child in &strong.children {
                collect_inline_spans(child, spans, bold_style);
            }
        }
        Node::Emphasis(emphasis) => {
            let italic_style = base_style.add_modifier(Modifier::ITALIC);
            for child in &emphasis.children {
                collect_inline_spans(child, spans, italic_style);
            }
        }
        Node::InlineCode(code) => {
            let code_style = base_style.fg(Color::Green).add_modifier(Modifier::BOLD);
            spans.push(Span::styled(code.value.clone(), code_style));
        }
        Node::Link(link) => {
            let link_style = base_style
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED);
            for child in &link.children {
                collect_inline_spans(child, spans, link_style);
            }
        }
        Node::Break(_) => {
            spans.push(Span::raw("\n"));
        }
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    collect_inline_spans(child, spans, base_style);
                }
            }
        }
    }
}

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
    match (key_code, modifiers) {
        // Single line scrolling
        (KeyCode::Char('j'), KeyModifiers::NONE) => {
            app.scroll_view_state.scroll_down();
        }
        (KeyCode::Char('k'), KeyModifiers::NONE) => {
            app.scroll_view_state.scroll_up();
        }
        // Slide navigation
        (KeyCode::Char('h'), KeyModifiers::NONE) => {
            if app.current_slide > 0 {
                app.current_slide -= 1;
                app.scroll_view_state = ScrollViewState::default();
            }
        }
        (KeyCode::Char('l'), KeyModifiers::NONE) => {
            if app.current_slide + 1 < app.slides.len() {
                app.current_slide += 1;
                app.scroll_view_state = ScrollViewState::default();
            }
        }
        // Page scrolling
        (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
            app.scroll_view_state.scroll_page_down();
        }
        (KeyCode::Char('b'), KeyModifiers::CONTROL) => {
            app.scroll_view_state.scroll_page_up();
        }
        // Half-page scrolling
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
            let mut offset = app.scroll_view_state.offset();
            let half_page = app.viewport_height / 2;
            offset.y = offset.y.saturating_add(half_page);
            app.scroll_view_state.set_offset(offset);
        }
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
            let mut offset = app.scroll_view_state.offset();
            let half_page = app.viewport_height / 2;
            offset.y = offset.y.saturating_sub(half_page);
            app.scroll_view_state.set_offset(offset);
        }
        // Jump to top/bottom
        (KeyCode::Char('g'), KeyModifiers::NONE) => {
            let mut offset = app.scroll_view_state.offset();
            offset.y = 0;
            app.scroll_view_state.set_offset(offset);
        }
        (KeyCode::Char('G'), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
            app.scroll_view_state.scroll_to_bottom();
        }
        _ => {}
    }
}

pub fn run_app(term: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let slides = demo()?;
    let mut app = App {
        slides,
        current_slide: 0,
        scroll_view_state: ScrollViewState::default(),
        viewport_height: 0,
    };
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
