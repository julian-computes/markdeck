use anyhow::{Result, anyhow};
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::commands::Command;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keymaps: Keymaps,
}

#[derive(Debug, Deserialize, Default)]
pub struct Keymaps {
    #[serde(default)]
    pub scroll_down: Vec<String>,
    #[serde(default)]
    pub scroll_up: Vec<String>,
    #[serde(default)]
    pub previous_slide: Vec<String>,
    #[serde(default)]
    pub next_slide: Vec<String>,
    #[serde(default)]
    pub page_down: Vec<String>,
    #[serde(default)]
    pub page_up: Vec<String>,
    #[serde(default)]
    pub half_page_down: Vec<String>,
    #[serde(default)]
    pub half_page_up: Vec<String>,
    #[serde(default)]
    pub jump_to_top: Vec<String>,
    #[serde(default)]
    pub jump_to_bottom: Vec<String>,
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self> {
        let config_path = if let Some(p) = path {
            PathBuf::from(p)
        } else {
            let mut default_path = dirs::config_dir()
                .ok_or_else(|| anyhow!("Could not determine config directory"))?;
            default_path.push("markdeck");
            default_path.push("config.toml");
            default_path
        };

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn get_command(&self, key_code: KeyCode, modifiers: KeyModifiers) -> Option<Command> {
        let key_str = keycode_to_string(key_code, modifiers);

        for binding in &self.keymaps.scroll_down {
            if binding == &key_str {
                return Some(Command::ScrollDown);
            }
        }
        for binding in &self.keymaps.scroll_up {
            if binding == &key_str {
                return Some(Command::ScrollUp);
            }
        }
        for binding in &self.keymaps.previous_slide {
            if binding == &key_str {
                return Some(Command::PreviousSlide);
            }
        }
        for binding in &self.keymaps.next_slide {
            if binding == &key_str {
                return Some(Command::NextSlide);
            }
        }
        for binding in &self.keymaps.page_down {
            if binding == &key_str {
                return Some(Command::PageDown);
            }
        }
        for binding in &self.keymaps.page_up {
            if binding == &key_str {
                return Some(Command::PageUp);
            }
        }
        for binding in &self.keymaps.half_page_down {
            if binding == &key_str {
                return Some(Command::HalfPageDown);
            }
        }
        for binding in &self.keymaps.half_page_up {
            if binding == &key_str {
                return Some(Command::HalfPageUp);
            }
        }
        for binding in &self.keymaps.jump_to_top {
            if binding == &key_str {
                return Some(Command::JumpToTop);
            }
        }
        for binding in &self.keymaps.jump_to_bottom {
            if binding == &key_str {
                return Some(Command::JumpToBottom);
            }
        }

        None
    }

    pub fn get_keys_for_command(&self, command: Command) -> Option<&str> {
        let bindings = match command {
            Command::ScrollDown => &self.keymaps.scroll_down,
            Command::ScrollUp => &self.keymaps.scroll_up,
            Command::PreviousSlide => &self.keymaps.previous_slide,
            Command::NextSlide => &self.keymaps.next_slide,
            Command::PageDown => &self.keymaps.page_down,
            Command::PageUp => &self.keymaps.page_up,
            Command::HalfPageDown => &self.keymaps.half_page_down,
            Command::HalfPageUp => &self.keymaps.half_page_up,
            Command::JumpToTop => &self.keymaps.jump_to_top,
            Command::JumpToBottom => &self.keymaps.jump_to_bottom,
        };

        bindings.first().map(|s| s.as_str())
    }

    pub fn format_help_text(&self) -> String {
        let mut parts = vec![];

        if let (Some(prev), Some(next)) = (
            self.get_keys_for_command(Command::PreviousSlide),
            self.get_keys_for_command(Command::NextSlide),
        ) {
            parts.push(format!("{}/{}: slides", prev, next));
        }

        if let (Some(down), Some(up)) = (
            self.get_keys_for_command(Command::ScrollDown),
            self.get_keys_for_command(Command::ScrollUp),
        ) {
            parts.push(format!("{}/{}: scroll", down, up));
        }

        if let (Some(down), Some(up)) = (
            self.get_keys_for_command(Command::HalfPageDown),
            self.get_keys_for_command(Command::HalfPageUp),
        ) {
            parts.push(format!("{}/{}: half page", down, up));
        }

        if let (Some(down), Some(up)) = (
            self.get_keys_for_command(Command::PageDown),
            self.get_keys_for_command(Command::PageUp),
        ) {
            parts.push(format!("{}/{}: full page", down, up));
        }

        if let (Some(top), Some(bottom)) = (
            self.get_keys_for_command(Command::JumpToTop),
            self.get_keys_for_command(Command::JumpToBottom),
        ) {
            parts.push(format!("{}/{}: top/bottom", top, bottom));
        }

        parts.push("q: quit".to_string());

        parts.join("  ")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keymaps: Keymaps {
                scroll_down: vec!["j".to_string(), "Down".to_string()],
                scroll_up: vec!["k".to_string(), "Up".to_string()],
                previous_slide: vec!["h".to_string()],
                next_slide: vec!["l".to_string()],
                page_down: vec!["C-f".to_string()],
                page_up: vec!["C-b".to_string()],
                half_page_down: vec!["C-d".to_string()],
                half_page_up: vec!["C-u".to_string()],
                jump_to_top: vec!["g".to_string()],
                jump_to_bottom: vec!["G".to_string()],
            },
        }
    }
}

fn keycode_to_string(key_code: KeyCode, modifiers: KeyModifiers) -> String {
    let base = match key_code {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        _ => return String::new(),
    };

    if modifiers.contains(KeyModifiers::CONTROL) {
        format!("C-{}", base)
    } else if modifiers.contains(KeyModifiers::ALT) {
        format!("A-{}", base)
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_j_scrolls_down() {
        let config = Config::default();
        let cmd = config.get_command(KeyCode::Char('j'), KeyModifiers::NONE);
        assert!(matches!(cmd, Some(Command::ScrollDown)));
    }

    #[test]
    fn test_default_config_down_arrow_scrolls_down() {
        let config = Config::default();
        let cmd = config.get_command(KeyCode::Down, KeyModifiers::NONE);
        assert!(matches!(cmd, Some(Command::ScrollDown)));
    }

    #[test]
    fn test_default_config_ctrl_f_pages_down() {
        let config = Config::default();
        let cmd = config.get_command(KeyCode::Char('f'), KeyModifiers::CONTROL);
        assert!(matches!(cmd, Some(Command::PageDown)));
    }

    #[test]
    fn test_keycode_to_string_char() {
        let s = keycode_to_string(KeyCode::Char('a'), KeyModifiers::NONE);
        assert_eq!(s, "a");
    }

    #[test]
    fn test_keycode_to_string_ctrl() {
        let s = keycode_to_string(KeyCode::Char('f'), KeyModifiers::CONTROL);
        assert_eq!(s, "C-f");
    }

    #[test]
    fn test_keycode_to_string_arrow() {
        let s = keycode_to_string(KeyCode::Down, KeyModifiers::NONE);
        assert_eq!(s, "Down");
    }

    #[test]
    fn test_format_help_text_default_config() {
        let config = Config::default();
        let help_text = config.format_help_text();
        assert!(help_text.contains("h/l: slides"));
        assert!(help_text.contains("j/k: scroll"));
        assert!(help_text.contains("C-d/C-u: half page"));
        assert!(help_text.contains("C-f/C-b: full page"));
        assert!(help_text.contains("g/G: top/bottom"));
        assert!(help_text.contains("q: quit"));
    }

    #[test]
    fn test_get_keys_for_command() {
        let config = Config::default();
        assert_eq!(config.get_keys_for_command(Command::ScrollDown), Some("j"));
        assert_eq!(config.get_keys_for_command(Command::ScrollUp), Some("k"));
        assert_eq!(config.get_keys_for_command(Command::NextSlide), Some("l"));
    }
}
