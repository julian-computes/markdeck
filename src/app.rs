use anyhow::{Result, anyhow};
use markdown::{ParseOptions, mdast::Node, to_mdast};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use tui_scrollview::ScrollViewState;

pub struct App {
    pub slides: Vec<Vec<Node>>,
    pub current_slide: usize,
    pub scroll_view_state: ScrollViewState,
    pub viewport_height: u16,
}

impl App {
    pub fn new(slides: Vec<Vec<Node>>) -> Self {
        Self {
            slides,
            current_slide: 0,
            scroll_view_state: ScrollViewState::default(),
            viewport_height: 0,
        }
    }
}

pub fn load_slides(path: &str) -> Result<Vec<Vec<Node>>> {
    let content = std::fs::read_to_string(path)?;
    let mut mdast =
        to_mdast(content.as_str(), &ParseOptions::default()).map_err(|e| anyhow!("{}", e))?;

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

pub fn node_to_lines(node: &Node, lines: &mut Vec<Line<'static>>, style: Style) {
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
