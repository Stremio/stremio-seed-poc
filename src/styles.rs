pub mod global;
pub mod themes;

use seed::*;
use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use themes::{Color, get_color_value};

pub fn button_container() -> Vec<Style> {
    vec![
        s()
            .outline_width(global::FOCUS_OUTLINE_SIZE)
            .outline_color(Color::SurfaceLighter)
            .raw(format!("outline-offset: calc(-1 * {});", global::FOCUS_OUTLINE_SIZE).as_str())
            .cursor(CssCursor::Pointer),
        s()
            .focus()
            .outline_style(CssOutlineStyle::Solid),
        s()
            .disabled()
            .pointer_events("none"),
    ]
}

pub fn text_input() -> Style {
    s()
        .user_select("text")
} 
