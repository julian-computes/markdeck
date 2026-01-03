use crate::app::App;
use tui_scrollview::ScrollViewState;

pub enum Command {
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    HalfPageDown,
    HalfPageUp,
    JumpToTop,
    JumpToBottom,
    NextSlide,
    PreviousSlide,
}

impl Command {
    pub fn execute(&self, app: &mut App) {
        match self {
            Command::ScrollDown => {
                app.scroll_view_state.scroll_down();
            }
            Command::ScrollUp => {
                app.scroll_view_state.scroll_up();
            }
            Command::PageDown => {
                app.scroll_view_state.scroll_page_down();
            }
            Command::PageUp => {
                app.scroll_view_state.scroll_page_up();
            }
            Command::HalfPageDown => {
                let mut offset = app.scroll_view_state.offset();
                let half_page = app.viewport_height / 2;
                offset.y = offset.y.saturating_add(half_page);
                app.scroll_view_state.set_offset(offset);
            }
            Command::HalfPageUp => {
                let mut offset = app.scroll_view_state.offset();
                let half_page = app.viewport_height / 2;
                offset.y = offset.y.saturating_sub(half_page);
                app.scroll_view_state.set_offset(offset);
            }
            Command::JumpToTop => {
                let mut offset = app.scroll_view_state.offset();
                offset.y = 0;
                app.scroll_view_state.set_offset(offset);
            }
            Command::JumpToBottom => {
                app.scroll_view_state.scroll_to_bottom();
            }
            Command::NextSlide => {
                if app.current_slide + 1 < app.slides.len() {
                    app.current_slide += 1;
                    app.scroll_view_state = ScrollViewState::default();
                }
            }
            Command::PreviousSlide => {
                if app.current_slide > 0 {
                    app.current_slide -= 1;
                    app.scroll_view_state = ScrollViewState::default();
                }
            }
        }
    }
}
