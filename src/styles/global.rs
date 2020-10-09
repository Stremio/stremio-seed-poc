use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use std::collections::HashMap;
use crate::styles::themes::{self, Breakpoint, Color};

pub fn init() {
    let landscape_shape_ratio = 0.5625;
    let poster_shape_ration = 1.464;
    let scroll_bar_width = px(6);
    let focus_outline_size = px(2);
    let color_facebook = "#4267b2";
    let color_twitter = "#1DA1F2";
    let color_placeholder = "#60606080";

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

    GlobalStyle::default()
        .style(
            "html",
            s()
                .width(pc(100))
                .height(pc(100))
                .min_width(pc(100))
                .min_height(pc(100))
                .font_family("'Roboto', 'sans-serif'")
                .overflow(CssOverflow::Auto)
        )
        .style(
            "html",
            s()
                .only_and_above(Breakpoint::XXLarge)
                .font_size(px(18))
        )
        .style(
            "html",
            s()
                .only_and_below(Breakpoint::XXLarge)
                .font_size(px(16))
        )
        .style(
            "html",
            s()
                .only_and_below(Breakpoint::Large)
                .font_size(px(15))
        )
        .style(
            "html",
            s()
                .only_and_below(Breakpoint::Medium)
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
        .add_font_faces()
        .activate_init_styles();
}

trait GlobalStyleExt {
    fn add_font_faces(self) -> Self;
}

impl GlobalStyleExt for GlobalStyle {
    fn add_font_faces(self) -> Self where Self: Sized {
        self
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("300")
                    .raw("src: url('/fonts/Roboto-LightItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("400")
                    .raw("src: url('/fonts/Roboto-RegularItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("500")
                    .raw("src: url('/fonts/Roboto-MediumItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Italic)
                    .font_weight("700")
                    .raw("src: url('/fonts/Roboto-BoldItalic.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("300")
                    .raw("src: url('/fonts/Roboto-Light.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("400")
                    .raw("src: url('/fonts/Roboto-Regular.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("500")
                    .raw("src: url('/fonts/Roboto-Medium.ttf') format('truetype');") 
            )
            .style(
                "@font-face",
                s()
                    .font_family("'Roboto'")
                    .font_style(CssFontStyle::Normal)
                    .font_weight("700")
                    .raw("src: url('/fonts/Roboto-Bold.ttf') format('truetype');") 
            )
    }
}
