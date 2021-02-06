use crate::{PageId, Msg, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};
use std::rc::Rc;

#[view]
pub fn menu_button(root_base_url: &Url) -> Node<Msg> {
    let active = true;
    label![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        IF!(active => s().background_color(Color::BackgroundLight2)),
        attrs!{
            At::TabIndex => -1,
        },
        ev(Ev::Click, |_| log!("menu_button clicked")),
        menu_icon(),
        menu_container(root_base_url),
    ]
}

#[view]
fn menu_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .overflow(CssOverflow::Visible)
            .fill(Color::SecondaryVariant2Light1_90)
            .flex(CssFlex::None)
            .height(rem(1.7))
            .width(rem(1.7)),
        attrs!{
            At::ViewBox => "0 0 216 1024",
            At::from("icon") => "ic_more",
        },
        path![
            attrs!{
                At::D => "M215.944 108.122c0-0.089 0-0.195 0-0.301 0-59.714-48.408-108.122-108.122-108.122s-108.122 48.408-108.122 108.122c0 59.714 48.408 108.122 108.122 108.122 0.106 0 0.211-0 0.317-0l-0.016 0c59.548 0 107.821-48.273 107.821-107.821v0z",
            },
        ],
        path![
            attrs!{
                At::D => "M215.944 507.181c-0-59.714-48.408-108.122-108.122-108.122s-108.122 48.408-108.122 108.122c0 59.714 48.408 108.122 108.122 108.122 0.106 0 0.212-0 0.318-0l-0.016 0c0 0 0 0 0 0 59.548 0 107.821-48.273 107.821-107.821 0-0.106-0-0.212-0-0.318l0 0.017z",
            },
        ],
        path![
            attrs!{
                At::D => "M215.944 915.878c-0-59.714-48.408-108.122-108.122-108.122s-108.122 48.408-108.122 108.122c0 59.714 48.408 108.122 108.122 108.122 0.106 0 0.212-0 0.318-0l-0.016 0c0 0 0 0 0 0 59.548 0 107.821-48.273 107.821-107.821 0-0.106-0-0.212-0-0.318l0 0.017z",
            },
        ],
    ]
}

#[view]
fn menu_container(root_base_url: &Url) -> Node<Msg> {
    div![
        C!["menu-container", "menu-direction-bottom-left"],
        s()
            .bottom("initial")
            .left("initial")
            .right("0")
            .top("100%")
            .visibility(CssVisibility::Visible)
            .box_shadow("0 1.35rem 2.7rem hsla(0,0%,0%,0.4),0 1.1rem 0.85rem hsla(0,0%,0%,0.2)")
            .cursor(CssCursor::Auto)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Absolute)
            .z_index("1"),
        div![
            C!["nav-menu-container"],
            s()
                .background_color(Color::BackgroundDark1)
                .max_height(format!("calc(100vh - {})", global::HORIZONTAL_NAV_BAR_SIZE).as_str())
                .overflow_y("auto")
                .width(rem(20)),
            menu_section_user(root_base_url),
            menu_section_fullscreen(),
            menu_section_general(root_base_url),
            menu_section_docs(),
        ]
    ]
}

#[view]
fn menu_section_user(root_base_url: &Url) -> Node<Msg> {
    div![
        C!["user-info-container"],
        s()
            .display(CssDisplay::Grid)
            .grid_template_areas(r#""avatar-area email-area" "avatar-area logout-button-area""#)
            .grid_template_columns("7rem 1fr")
            .grid_template_rows("50% 50%")
            .height(rem(7)),
        div![
            C!["avatar-container"],
            s()
                .background_image(format!(r#"url("{}")"#, global::image_url("anonymous.png")).as_str())
                .background_clip("content-box")
                .background_origin("content-box")
                .background_position(CssBackgroundPosition::Center)
                .background_repeat(CssBackgroundRepeat::NoRepeat)
                .background_size("cover")
                .border_radius(pc(50))
                .grid_area("avatar-area")
                .opacity("0.9")
                .padding(rem(1)),
        ],
        div![
            C!["email-container"],
            s()
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .grid_area("email-area")
                .padding("1rem 1rem 0 0"),
            div![
                C!["email-label"],
                s()
                    .color(Color::SurfaceLight5_90)
                    .flex("1")
                    .max_height(em(2.4)),
                "Anonymous user",
            ]
        ],
        a![
            C!["logout-button-container", "button-container"],
            attrs!{
                At::TabIndex => 0,
                At::Title => "Log in / Sign up",
                At::Href => RootUrls::new(root_base_url).intro(),
            },
            s()
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .grid_area("logout-button-area")
                .padding("0 1rem 1rem 0")
                .cursor(CssCursor::Pointer),
            div![
                C!["logout-label"],
                s()
                    .color(Color::SurfaceLight3_90)
                    .flex("1")
                    .max_height(em(2.4)),
                s()
                    .hover()
                    .color(Color::SurfaceLight5_90)
                    .text_decoration(CssTextDecoration::Underline),
                "Log in / Sign up",
            ]
        ]
    ]
}

#[view]
fn menu_section_fullscreen() -> Node<Msg> {
    div![
        C!["nav-menu-section"],
        s()
            .border_top("thin solid hsla(0,0%,100%,0.2)"),
        menu_option(MenuOptionArgs { 
            title: "Enter Fullscreen", url: "", target_blank: false 
        }),
    ]
}

#[view]
fn menu_section_general(root_base_url: &Url) -> Node<Msg> {
    div![
        C!["nav-menu-section"],
        s()
            .border_top("thin solid hsla(0,0%,100%,0.2)"),
        menu_option(MenuOptionArgs { 
            title: "Settings", url: &RootUrls::new(root_base_url).settings().to_string(), target_blank: false 
        }),
        menu_option(MenuOptionArgs { 
            title: "Addons", url: &RootUrls::new(root_base_url).addons_urls().root().to_string(), target_blank: false 
        }),
        menu_option(MenuOptionArgs { 
            title: "Remote Control", url: "", target_blank: false 
        }),
        menu_option(MenuOptionArgs { 
            title: "Play Magnet Link", url: "", target_blank: false  
        }),
        menu_option(MenuOptionArgs { 
            title: "Helps & Feedback", url: "https://stremio.zendesk.com/", target_blank: true 
        }),
    ]
}

#[view]
fn menu_section_docs() -> Node<Msg> {
    div![
        C!["nav-menu-section"],
        s()
            .border_top("thin solid hsla(0,0%,100%,0.2)"),
        menu_option(MenuOptionArgs { 
            title: "Terms of Service", url: "https://www.stremio.com/tos", target_blank: true 
        }),
        menu_option(MenuOptionArgs { 
            title: "Privacy Policy", url: "https://www.stremio.com/privacy", target_blank: true 
        }),
        menu_option(MenuOptionArgs { 
            title: "About Stremio", url: "https://www.stremio.com/", target_blank: true 
        }),
    ]
}

struct MenuOptionArgs<'a> {
    title: &'a str,
    url: &'a str,
    target_blank: bool,
}

#[view]
fn menu_option(args: MenuOptionArgs) -> Node<Msg> {
    a![
        C!["nav-menu-option-container", "button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(rem(4))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        attrs!{
            At::TabIndex => 0,
            At::Title => args.title,
            At::Href => args.url,
        },
        IF!(args.target_blank => {
            attrs!{
                At::Target => "_blank",
            }
        }),
        div![
            C!["nav-menu-option-label"],
            s()
                .padding_left(rem(1.3))
                .color(Color::SurfaceLight5_90)
                .flex("1")
                .max_height(em(2.4))
                .padding_right(rem(1.3)),
            args.title,
        ]
    ]
}
