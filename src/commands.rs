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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_slide_within_bounds() {
        let mut app = App::new(vec![vec![], vec![], vec![]]);
        Command::NextSlide.execute(&mut app);
        assert_eq!(app.current_slide, 1);
    }

    #[test]
    fn test_next_slide_at_last_slide_does_nothing() {
        let mut app = App::new(vec![vec![], vec![]]);
        app.current_slide = 1;
        Command::NextSlide.execute(&mut app);
        assert_eq!(app.current_slide, 1);
    }

    #[test]
    fn test_previous_slide_within_bounds() {
        let mut app = App::new(vec![vec![], vec![], vec![]]);
        app.current_slide = 2;
        Command::PreviousSlide.execute(&mut app);
        assert_eq!(app.current_slide, 1);
    }

    #[test]
    fn test_previous_slide_at_first_slide_does_nothing() {
        let mut app = App::new(vec![vec![], vec![]]);
        app.current_slide = 0;
        Command::PreviousSlide.execute(&mut app);
        assert_eq!(app.current_slide, 0);
    }

    #[test]
    fn test_next_slide_resets_scroll_state() {
        let mut app = App::new(vec![vec![], vec![]]);
        let mut offset = app.scroll_view_state.offset();
        offset.y = 10;
        app.scroll_view_state.set_offset(offset);

        Command::NextSlide.execute(&mut app);

        let new_offset = app.scroll_view_state.offset();
        assert_eq!(new_offset.y, 0);
    }

    #[test]
    fn test_previous_slide_resets_scroll_state() {
        let mut app = App::new(vec![vec![], vec![]]);
        app.current_slide = 1;
        let mut offset = app.scroll_view_state.offset();
        offset.y = 10;
        app.scroll_view_state.set_offset(offset);

        Command::PreviousSlide.execute(&mut app);

        let new_offset = app.scroll_view_state.offset();
        assert_eq!(new_offset.y, 0);
    }
}
