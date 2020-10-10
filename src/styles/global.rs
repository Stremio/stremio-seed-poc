use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use std::collections::HashMap;
use crate::styles::themes::{self, Breakpoint, Color};

mod font_faces;
use font_faces::GlobalStyleFontFaces;

pub const landscape_shape_ratio: f64 = 0.5625;
pub const poster_shape_ration: f64 = 1.464;
pub const scroll_bar_width: &str = "6px";
pub const focus_outline_size: &str = "2px";
pub const color_facebook: &str = "#4267b2";
pub const color_twitter: &str = "#1DA1F2";
pub const color_placeholder: &str = "#60606080";

pub fn init() {
    load_app_themes(&[themes::default_color_theme, themes::default_breakpoint_theme]);

    let get_color_value = |color: Color| {
        app_themes().get_with(|themes| {
            themes
                .iter()
                .find(|theme| theme.name == "default_color_theme")
                .map(|theme| {
                    let colors = theme
                        .anymap.get::<HashMap<Color, CssColor>>().unwrap();
                    let css_color = colors.get(&color).unwrap();
                    let string_color = css_color.to_string();
                    string_color
                        .strip_prefix("color: ").unwrap()
                        .strip_suffix(";").unwrap()
                        .to_owned()
                })
        }).unwrap()
    };

    GlobalStyle::new()
        .add_font_faces()
        .style(
            "html",
            s()
                .width(pc(100))
                .height(pc(100))
                .min_width(px(800))
                .min_height(px(600))
                .font_family("'Roboto', 'sans-serif'")
                .overflow(CssOverflow::Auto)
        )
        .style(
            "html",
            s()
                .only_and_above(Breakpoint::XLarge)
                .font_size(px(18))
        )
        .style(
            "html",
            s()
                .only_and_below(Breakpoint::XLarge)
                .font_size(px(16))
        )
        .style(
            "html",
            s()
                .only_and_below(Breakpoint::Medium)
                .font_size(px(15))
        )
        .style(
            "html",
            s()
                .only_and_below(Breakpoint::Small)
                .font_size(px(14))
        )
        .style(
            "body",
            s()
                .width(pc(100))
                .height(pc(100))
        )
        .style(
            "svg",
            s()
                .overflow(CssOverflow::Visible)
        )
        .style(
            "::-webkit-scrollbar",
            s()
                .width(scroll_bar_width)
        )
        .style(
            "::-webkit-scrollbar-thumb",
            s()
                .background_color(Color::SecondaryLighter80)
        )
        .style(
            "::-webkit-scrollbar-track",
            s()
            .background_color(Color::SecondaryLight)
        )
        .style(
            "*",
            s()
                .margin("0")
                .padding("0")
                .box_sizing(CssBoxSizing::BorderBox)
                .font_size(rem(1))
                .line_height(rem(1.2))
                .font_family(CssFontFamily::Inherit)
                .border("none")
                .outline(CssOutline::None)
                .list_style("none")
                .user_select("none")
                .text_decoration(CssTextDecoration::None)
                .raw("appearance: none;")
                .background("none")
                .box_shadow(CssBoxShadow::None)
                .overflow(CssOverflow::Hidden)
                .word_break("break-word")
                .raw("scrollbar-width: thin;")
                .raw(format!(
                    "scrollbar-color: {} {};", 
                    get_color_value(Color::SecondaryLighter80), 
                    get_color_value(Color::BackgroundLight)
                ).as_str())
        )
        .style(
            "#app",
            s()
                .position(CssPosition::Relative)
                .z_index(CssZIndex::Integer(0))
                .width(pc(100))
                .height(pc(100))
        )
        .activate_init_styles();
}
