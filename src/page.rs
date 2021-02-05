pub mod addons;
pub mod board;
pub mod detail;
pub mod discover;
pub mod intro;
pub mod library;
pub mod not_found;
pub mod player;
pub mod search;
pub mod settings;
pub mod test_links;

use crate::{PageId, Msg, Urls as RootUrls, Context};
use seed::{prelude::*, *};
use seed_styles::{pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};
use std::rc::Rc;

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!"); })
}

pub struct BasicLayoutArgs<'a> {
    page_content: Node<Msg>,
    container_class: &'a str,
    context: &'a Context,
    page_id: PageId,
    search_args: Option<SearchArgs<'a>>,
}

pub struct SearchArgs<'a> {
    input_search_query: &'a str,
    on_search_query_input_changed: Rc<dyn Fn(String) -> Msg>,
    on_search: Rc<dyn Fn() -> Msg>,
}

#[view]
pub fn basic_layout(args: BasicLayoutArgs) -> Node<Msg> {
    div![
        C!["route-content"],
        s()
            .position(CssPosition::Absolute)
            .bottom("0")
            .left("0")
            .right("0")
            .top("0")
            .overflow(CssOverflow::Hidden)
            .z_index("0"),
        div![
            C![args.container_class, "main-nav-bars-container"],
            s()
                .background_color(Color::BackgroundDark2)
                .height(pc(100))
                .width(pc(100))
                .position(CssPosition::Relative)
                .z_index("0"),
            horizontal_nav_bar(&args.context.root_base_url, args.search_args.as_ref()),
            vertical_nav_bar(&args.context.root_base_url, args.page_id),
            nav_content_container(args.page_content),
        ]
    ]
}

#[view]
fn horizontal_nav_bar(root_base_url: &Url, search_args: Option<&SearchArgs>) -> Node<Msg> {
    nav![
        C!["horizontal-nav-bar", "horizontal-nav-bar-container"],
        s()
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("1")
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .overflow(CssOverflow::Visible)
            .padding_right(rem(1)),
        logo_container(),
        spacer(None),
        search_bar(search_args),
        spacer(Some("11rem")),
        addons_top_button(root_base_url),
        fullscreen_button(),
        menu_button(root_base_url),
    ]
}

#[view]
fn logo_container() -> Node<Msg> {
    div![
        C!["logo-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::VERTICAL_NAV_BAR_SIZE),
        logo(),
    ]
}

#[view]
fn logo() -> Node<Msg> {
    img![
        C!["logo"],
        s()
            .flex(CssFlex::None)
            .height(rem(2.5))
            .object_fit("contain")
            .opacity("0.9")
            .width(rem(2.5)),
        attrs!{
            At::Src => global::image_url("stremio_symbol.png"),
        }
    ]
}

#[view]
fn spacer(max_width: Option<&str>) -> Node<Msg> {
    div![
        C!["spacing"],
        s()
            .flex("1 0 0"),
        max_width.map(|max_width| {
            s()
                .max_width(max_width)
        }),
    ]
}

#[view]
fn search_bar(search_args: Option<&SearchArgs>) -> Node<Msg> {
    label![
        C!["search-bar", "search-bar-container"],
        s()
            .flex("2 0 9.5rem")
            .max_width(rem(30))
            .background_color(Color::BackgroundLight2)
            .border_radius(global::SEARCH_BAR_SIZE)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::SEARCH_BAR_SIZE),
        s()
            .hover()
            .background_color(Color::BackgroundLight3),
        ev(Ev::Click, |_| Msg::GoToSearchPage),
        search_input(search_args),
        search_button(search_args),
    ]
}

#[view]
fn search_input(search_args: Option<&SearchArgs>) -> Node<Msg> {
    input![
        C!["search-input", "text-input"],
        s()
            .style_other("::placeholder")
            .color(Color::SecondaryVariant1Light1_90)
            .max_height(em(1.2))
            .opacity("1"),
        s()
            .user_select("text")
            .align_items(CssAlignItems::Center)
            .align_self(CssAlignSelf::Stretch)
            .color(Color::SecondaryVariant1Light1_90)
            .display(CssDisplay::Flex)
            .flex("1")
            .flex_direction(CssFlexDirection::Row)
            .font_weight("500")
            .padding("0 0.5rem 0 1.5rem"),
        attrs!{
            At::from("autocorrect") => "off",
            At::from("autocapitalize") => "off",
            At::AutoComplete => "off",
            At::SpellCheck => "false",
            At::TabIndex => -1,
            At::Type => "text",
            At::Placeholder => "Search or paste link",
            At::Value => search_args.map(|args| args.input_search_query).unwrap_or_default(),
        },
        search_args.map(|args| {
            let on_change = Rc::clone(&args.on_search_query_input_changed);
            input_ev(Ev::Input, move |query| on_change(query))
        }),
    ]
}

#[view]
fn search_button(search_args: Option<&SearchArgs>) -> Node<Msg> {
    div![
        C!["submit-button-container", "button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .height(global::SEARCH_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::SEARCH_BAR_SIZE)
            .cursor(CssCursor::Pointer),
        attrs!{
            At::TabIndex => -1,
        },
        search_args.map(|args| {
            let on_search = Rc::clone(&args.on_search);
            ev(Ev::Click, move |_| on_search())
        }),
        search_icon(),
    ]
}

#[view]
fn search_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .overflow(CssOverflow::Visible)
            .fill(Color::SecondaryVariant1_90)
            .flex(CssFlex::None)
            .height(rem(1.7))
            .width(rem(1.7)),
        attrs!{
            At::ViewBox => "0 0 1443 1024",
            At::from("icon") => "ic_search_link",
        },
        path![
            attrs!{
                At::D => "M1033.035 774.927h-105.111c-0.013 0-0.027 0-0.042 0-10.802 0-21.14-1.988-30.667-5.619l0.591 0.198c-15.423-5.707-27.932-16.268-35.965-29.798l-0.176-0.32c-2.484-3.967-4.719-8.539-6.464-13.345l-0.162-0.509c-3.048-7.589-4.817-16.388-4.819-25.599l-0-0.001c0.67-42.233 35.063-76.212 77.393-76.212 0.533 0 1.064 0.005 1.594 0.016l-0.079-0.001h144.264c0.863-0.033 1.877-0.052 2.896-0.052 7.433 0 14.63 1.008 21.462 2.896l-0.565-0.133c11.866 3.986 21.976 10.503 30.094 18.95l0.023 0.024c13.553 13.793 21.92 32.721 21.92 53.602 0 3.187-0.195 6.328-0.573 9.412l0.037-0.37c-0.198 1.162-0.312 2.5-0.312 3.864 0 6.594 2.649 12.569 6.94 16.92l-0.003-0.003c3.716 3.783 8.767 6.245 14.389 6.622l0.068 0.004c0.278 0.011 0.605 0.018 0.932 0.018 13.056 0 23.716-10.256 24.364-23.151l0.002-0.058c0.649-4.698 1.020-10.125 1.020-15.64 0-33.301-13.512-63.447-35.352-85.253l-0.001-0.001c-21.066-21.097-50.071-34.263-82.15-34.635l-0.071-0.001c-52.104 0-103.906 0-156.009 0-49.554 2.528-91.243 33.695-109.027 77.175l-0.3 0.83c-2.498 6.628-4.885 14.795-6.704 23.173l-0.223 1.222c-2.090 8.002-3.29 17.188-3.29 26.654s1.2 18.652 3.456 27.414l-0.166-0.76c-0.065 0.722-0.103 1.561-0.103 2.409s0.037 1.688 0.11 2.517l-0.008-0.107c0 2.711 2.108 5.722 3.313 8.433 0.933 2.58 1.948 4.765 3.126 6.846l-0.115-0.22 3.614 7.228c1.752 3.103 3.546 5.761 5.523 8.266l-0.102-0.134c1.236 2.097 2.429 3.867 3.716 5.561l-0.102-0.14c3.598 4.93 7.154 9.25 10.937 13.356l-0.094-0.104c0.859 1.159 1.853 2.153 2.974 2.985l0.038 0.027c18.807 19.502 44.944 31.827 73.961 32.525l0.129 0.002c40.056 1.506 80.113 0 120.471 0 0.263 0.011 0.571 0.017 0.881 0.017 9.895 0 18.303-6.362 21.359-15.218l0.048-0.159c1.655-2.99 2.629-6.556 2.629-10.35 0-4.964-1.668-9.539-4.474-13.194l0.038 0.051c-4.974-5.048-11.885-8.176-19.527-8.176-0.547 0-1.090 0.016-1.63 0.048l0.074-0.003z",
            },
        ],
        path![
            attrs!{
                At::D => "M1407.398 611.689l-3.012-3.012c-17.962-18.55-42.498-30.641-69.842-32.509l-0.332-0.018c-19.576-1.506-39.454 0-60.235 0s-42.767 0-64.151 0c-0.38-0.022-0.825-0.035-1.273-0.035-9.786 0-18.157 6.062-21.562 14.636l-0.055 0.157c-1.435 2.772-2.276 6.052-2.276 9.528 0 5.366 2.005 10.264 5.307 13.986l-0.019-0.022 1.506 1.807c5.195 4.38 11.964 7.042 19.355 7.042 0.926 0 1.843-0.042 2.748-0.124l-0.117 0.009h104.508c0.17-0.001 0.37-0.002 0.571-0.002 21.491 0 40.967 8.624 55.157 22.6l-0.010-0.010c13.214 13.239 21.385 31.515 21.385 51.699 0 0.142-0 0.284-0.001 0.426l0-0.022c-0.842 42.098-35.167 75.902-77.388 75.902-0.323 0-0.645-0.002-0.967-0.006l0.049 0h-145.468c-0.821 0.030-1.785 0.047-2.754 0.047-7.045 0-13.88-0.896-20.399-2.58l0.565 0.124c-12.291-3.615-22.831-9.967-31.328-18.378l0.006 0.006c-13.459-13.864-21.756-32.803-21.756-53.68 0-3.586 0.245-7.115 0.719-10.571l-0.045 0.401c0.377-1.787 0.592-3.84 0.592-5.943 0-6.983-2.376-13.411-6.365-18.519l0.050 0.067c-1.77-2.045-3.862-3.753-6.208-5.060l-0.116-0.060c-16.264-6.626-30.118 3.614-33.129 23.793-0.783 5.16-1.23 11.115-1.23 17.173 0 66.534 53.937 120.471 120.471 120.471 0.433 0 0.865-0.002 1.296-0.007l-0.066 0.001c49.995 0 99.991 0 150.588 0 50.623-0.695 93.946-31.236 113.227-74.793l0.317-0.802c6.184-13.844 9.785-30.001 9.785-46.998 0-34.274-14.642-65.128-38.013-86.649l-0.083-0.075z",
            },
        ],
        path![
            attrs!{
                At::D => "M992.075 865.882c-25.6 0-51.802 0-78.005 0-40.714-1.196-77.196-18.374-103.573-45.445l-0.031-0.032-3.614-3.915c-28.592-29.766-46.199-70.27-46.199-114.887 0-60.965 32.875-114.252 81.865-143.1l0.777-0.423c12.528-38.704 19.791-83.241 19.878-129.462l0-0.044c-1.371-237.151-193.936-428.869-431.278-428.869-238.192 0-431.285 193.093-431.285 431.285 0 237.342 191.718 429.907 428.738 431.277l0.131 0.001c0.118 0 0.258 0 0.397 0 88.033 0 169.923-26.302 238.24-71.477l-1.612 1.002 200.885 202.089c13.51 18.524 35.139 30.425 59.548 30.425 2.363 0 4.699-0.112 7.005-0.33l-0.295 0.023c1.429 0.081 3.101 0.127 4.784 0.127 35.359 0 65.974-20.311 80.814-49.902l0.237-0.521c7.55-11.025 12.058-24.651 12.058-39.33 0-20.085-8.438-38.2-21.963-50.992l-0.033-0.031zM433.694 736.376c-166.335 0-301.176-134.841-301.176-301.176v0-7.529c1.449-166.068 136.41-300.133 302.682-300.133 167.173 0 302.693 135.52 302.693 302.693 0 0.9-0.004 1.799-0.012 2.698l0.001-0.138c-1.855 167.126-137.013 302.072-304.044 303.585l-0.144 0.001z",
            },
        ],
    ]
}

#[view]
fn addons_top_button(root_base_url: &Url) -> Node<Msg> {
    a![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        attrs!{
            At::TabIndex => -1,
            At::Title => "Addons",
            At::Href => RootUrls::new(root_base_url).addons_urls().root(),
        },
        addons_top_icon(),
    ]
}

#[view]
fn addons_top_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .overflow(CssOverflow::Visible)
            .fill(Color::SecondaryVariant2Light1_90)
            .flex(CssFlex::None)
            .height(rem(1.7))
            .width(rem(1.7)),
        attrs!{
            At::ViewBox => "0 0 1043 1024",
            At::from("icon") => "ic_addons",
        },
        path![
            attrs!{
                At::D => "M145.468 679.454c-40.056-39.454-80.715-78.908-120.471-118.664-33.431-33.129-33.129-60.235 0-90.353l132.216-129.807c5.693-5.938 12.009-11.201 18.865-15.709l0.411-0.253c23.492-15.059 41.864-7.529 48.188 18.974 0 7.228 2.711 14.758 3.614 22.287 3.801 47.788 37.399 86.785 82.050 98.612l0.773 0.174c10.296 3.123 22.128 4.92 34.381 4.92 36.485 0 69.247-15.94 91.702-41.236l0.11-0.126c24.858-21.654 40.48-53.361 40.48-88.718 0-13.746-2.361-26.941-6.701-39.201l0.254 0.822c-14.354-43.689-53.204-75.339-99.907-78.885l-0.385-0.023c-18.372-2.409-41.562 0-48.188-23.492s11.445-34.635 24.998-47.887q65.054-62.946 130.409-126.795c32.527-31.925 60.235-32.226 90.353 0 40.659 39.153 80.715 78.908 120.471 118.362 8.348 8.594 17.297 16.493 26.82 23.671l0.587 0.424c8.609 7.946 20.158 12.819 32.846 12.819 24.823 0 45.29-18.653 48.148-42.707l0.022-0.229c3.012-13.252 4.518-26.805 8.734-39.755 12.103-42.212 50.358-72.582 95.705-72.582 3.844 0 7.637 0.218 11.368 0.643l-0.456-0.042c54.982 6.832 98.119 49.867 105.048 104.211l0.062 0.598c0.139 1.948 0.218 4.221 0.218 6.512 0 45.084-30.574 83.026-72.118 94.226l-0.683 0.157c-12.348 3.915-25.299 5.722-37.948 8.433-45.779 9.638-60.235 46.984-30.118 82.824 15.265 17.569 30.806 33.587 47.177 48.718l0.409 0.373c31.925 31.925 64.452 62.946 96.075 94.871 13.698 9.715 22.53 25.511 22.53 43.369s-8.832 33.655-22.366 43.259l-0.164 0.111c-45.176 45.176-90.353 90.353-137.035 134.325-5.672 5.996-12.106 11.184-19.169 15.434l-0.408 0.227c-4.663 3.903-10.725 6.273-17.341 6.273-13.891 0-25.341-10.449-26.92-23.915l-0.012-0.127c-2.019-7.447-3.714-16.45-4.742-25.655l-0.077-0.848c-4.119-47.717-38.088-86.476-82.967-97.721l-0.76-0.161c-9.584-2.63-20.589-4.141-31.947-4.141-39.149 0-74.105 17.956-97.080 46.081l-0.178 0.225c-21.801 21.801-35.285 51.918-35.285 85.185 0 1.182 0.017 2.36 0.051 3.533l-0.004-0.172c1.534 53.671 40.587 97.786 91.776 107.115l0.685 0.104c12.649 2.409 25.901 3.313 38.249 6.626 22.588 6.325 30.118 21.685 18.372 41.864-4.976 8.015-10.653 14.937-17.116 21.035l-0.051 0.047c-44.875 44.574-90.353 90.353-135.228 133.12-10.241 14.067-26.653 23.106-45.176 23.106s-34.935-9.039-45.066-22.946l-0.111-0.159c-40.659-38.852-80.414-78.908-120.471-118.362z",
            }
        ]
    ]
}

#[view]
fn fullscreen_button() -> Node<Msg> {
    div![
        C!["button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .justify_content(CssJustifyContent::Center)
            .width(global::HORIZONTAL_NAV_BAR_SIZE)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        attrs!{
            At::TabIndex => -1,
            At::Title => "Enter Fullscreen",
        },
        fullscreen_icon(),
        on_click_not_implemented(),
    ]
}

#[view]
fn fullscreen_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .overflow(CssOverflow::Visible)
            .fill(Color::SecondaryVariant2Light1_90)
            .flex(CssFlex::None)
            .height(rem(1.7))
            .width(rem(1.7)),
        attrs!{
            At::ViewBox => "0 0 1016 1024",
            At::from("icon") => "ic_fullscreen",
        },
        path![
            attrs!{
                At::D => "M379.784 1.506l-316.235-1.506c-17.58 0.003-33.524 7.011-45.19 18.385l0.014-0.013c-11.345 11.55-18.354 27.393-18.372 44.872l-0 0.003 1.506 316.838c0.663 34.993 28.856 63.187 63.787 63.848l0.063 0.001c0.090 0 0.196 0.001 0.302 0.001 34.492 0 62.473-27.876 62.644-62.328l0-0.016v-253.591h252.386c0.271 0.004 0.59 0.007 0.91 0.007 34.598 0 62.645-28.047 62.645-62.645 0-0.32-0.002-0.639-0.007-0.958l0.001 0.048c-1.004-34.88-29.443-62.792-64.437-62.946l-0.015-0z",
            },
        ],
        path![
            attrs!{
                At::D => "M633.976 128.904h254.494v252.386c-0.004 0.269-0.007 0.586-0.007 0.904 0 34.598 28.047 62.645 62.645 62.645 0.002 0 0.005-0 0.007-0l-0 0c35.122-0.497 63.483-28.753 64.15-63.787l0.001-0.063v-316.838c0.019-0.581 0.030-1.264 0.030-1.95 0-16.946-6.54-32.364-17.233-43.869l0.037 0.040c-11.448-11.329-27.189-18.338-44.568-18.372l-0.007-0-317.139 1.506c-35.189 0.334-63.646 28.686-64.15 63.802l-0.001 0.048c-0.004 0.271-0.007 0.59-0.007 0.91 0 34.282 27.538 62.133 61.7 62.638l0.048 0.001z",
            },
        ],
        path![
            attrs!{
                At::D => "M380.386 895.096h-252.386v-252.386c0.005-0.282 0.007-0.616 0.007-0.95 0-33.753-26.694-61.271-60.122-62.595l-0.12-0.004c-0.448-0.011-0.976-0.018-1.506-0.018-35.762 0-64.753 28.991-64.753 64.753 0 0.006 0 0.012 0 0.018l-0-0.001-1.506 316.838c-0.002 0.18-0.003 0.392-0.003 0.605 0 34.387 27.706 62.303 62.013 62.642l0.032 0h317.139c35.189-0.334 63.646-28.686 64.15-63.802l0.001-0.048c-0.142-35.138-27.992-63.725-62.825-65.050l-0.121-0.004z",
            },
        ],
        path![
            attrs!{
                At::D => "M950.814 580.066c-0.002-0-0.004-0-0.007-0-34.598 0-62.645 28.047-62.645 62.645 0 0.318 0.002 0.635 0.007 0.951l-0.001-0.048v252.386h-252.687c-0.18-0.002-0.392-0.003-0.605-0.003-34.387 0-62.303 27.706-62.642 62.013l-0 0.032c-0.007 0.359-0.011 0.783-0.011 1.207 0 35.554 28.655 64.416 64.13 64.75l0.032 0h316.536c17.385-0.034 33.126-7.043 44.58-18.377l-0.005 0.005c11.345-11.55 18.354-27.393 18.372-44.872l0-0.003v-316.838c-0.677-35.406-29.538-63.849-65.043-63.849-0.004 0-0.008 0-0.012 0l0.001-0z",
            },
        ],
    ]
}

#[view]
fn menu_button(root_base_url: &Url) -> Node<Msg> {
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

#[view]
fn vertical_nav_bar(root_base_url: &Url, page_id: PageId) -> Node<Msg> {
    nav![
        C!["vertical-nav-bar", "vertical-nav-bar-container"],
        s()
            .bottom("0")
            .left("0")
            .position(CssPosition::Absolute)
            .top(global::HORIZONTAL_NAV_BAR_SIZE)
            .z_index("1")
            .background_color(Color::BackgroundDark1)
            .overflow_y(CssOverflowY::Auto)
            .raw("scrollbar-width: none;")
            .width(global::VERTICAL_NAV_BAR_SIZE),
        vertical_nav_buttons(root_base_url, page_id),
    ]
}

#[view]
fn vertical_nav_buttons(root_base_url: &Url, page_id: PageId) -> Vec<Node<Msg>> {
    vec![
        { 
            let active = page_id == PageId::Board;
            vertical_nav_button(
                "Board", 
                active,
                RootUrls::new(root_base_url).root(), 
                true, 
                vertical_nav_icon("ic_board", "0 0 1395 1024", active, vec![
                    path![attrs!{At::D => "M1308.009 174.381l-1220.668 0c-48.237 0-87.341-39.104-87.341-87.341v0.301c0-48.237 39.104-87.341 87.341-87.341l1220.668-0c48.237 0 87.341 39.104 87.341 87.341v-0.301c0 48.237-39.104 87.341-87.341 87.341z"}],
                    path![attrs!{At::D => "M936.358 599.341l-849.016 0c-48.237 0-87.341-39.104-87.341-87.341v0.301c0-48.237 39.104-87.341 87.341-87.341l849.016-0c48.237 0 87.341 39.104 87.341 87.341v-0.301c0 48.237-39.104 87.341-87.341 87.341z"}],
                    path![attrs!{At::D => "M1308.009 1024h-1220.668c-48.237-0-87.341-39.104-87.341-87.341v0.301c0-48.237 39.104-87.341 87.341-87.341l1220.668 0c48.237 0 87.341 39.104 87.341 87.341v-0.301c0 48.237-39.104 87.341-87.341 87.341z"}],
                ])
            )
        },
        {
            let active = page_id == PageId::Discover;
            vertical_nav_button(
                "Discover", 
                active,
                RootUrls::new(root_base_url).discover_urls().root(), 
                false, 
                vertical_nav_icon("ic_discover", "0 0 1025 1024", active, vec![
                    path![attrs!{At::D => "M602.353 575.849c49.694-96.075 99.991-192.151 150.588-288.226 3.012-6.024 10.842-13.252 4.819-19.275s-13.553 0-19.275 4.216l-291.84 150.588c-10.241 5.534-18.27 14.048-23.055 24.371l-0.135 0.326q-64.753 124.386-129.506 248.471c-8.734 16.866-17.468 33.129-25.901 49.995-2.711 4.819-6.024 11.445 4.518 12.951 4.819-2.108 10.24-4.216 15.36-6.927l289.732-150.588c10.78-5.894 19.287-14.788 24.546-25.559l0.151-0.342z"}],
                    path![attrs!{At::D => "M883.351 161.732c-90.543-95.747-216.891-156.82-357.52-161.708l-0.88-0.024c-3.070-0.066-6.687-0.104-10.314-0.104-138.198 0-263.562 54.947-355.438 144.186l0.123-0.119c-98.26 92.852-159.424 224.071-159.424 369.575 0 142.717 58.843 271.691 153.591 363.984l0.111 0.107c88.622 88.958 210.672 144.561 345.709 146.368l0.343 0.004h24.094c277.633-5.364 500.641-231.69 500.641-510.104 0-136.661-53.732-260.772-141.221-352.36l0.185 0.195zM242.748 783.059c-70.126-69.135-113.568-165.177-113.568-271.364 0-210.414 170.574-380.988 380.988-380.988 0.644 0 1.288 0.002 1.931 0.005l-0.099-0c210.913 0 381.892 170.979 381.892 381.892s-170.979 381.892-381.892 381.892v0 0c-0.446 0.002-0.975 0.003-1.503 0.003-104.66 0-199.368-42.605-267.728-111.418l-0.020-0.021z"}],
                ])
            )
        },
        {
            let active = false;
            vertical_nav_button(
                "Library", 
                active,
                RootUrls::new(root_base_url).library(), 
                false, 
                vertical_nav_icon("ic_library", "0 0 1209 1024", active, vec![
                    path![attrs!{At::D => "M1204.706 917.082l-190.645-826.729c-9.055-39.42-43.838-68.374-85.384-68.374-48.324 0-87.499 39.175-87.499 87.499 0 6.779 0.771 13.378 2.23 19.714l-0.114-0.589 191.548 827.633c11.135 36.317 44.369 62.266 83.664 62.266 48.237 0 87.341-39.104 87.341-87.341 0-4.971-0.415-9.846-1.213-14.591l0.071 0.513z"}],
                    path![attrs!{At::D => "M674.334 0c-0-0-0-0-0.001-0-48.071 0-87.040 38.969-87.040 87.040 0 0.106 0 0.212 0.001 0.318l-0-0.016v849.318c-0.096 1.532-0.151 3.323-0.151 5.127 0 48.237 39.104 87.341 87.341 87.341s87.341-39.104 87.341-87.341c0-1.804-0.055-3.594-0.162-5.371l0.012 0.244v-849.318c0-48.237-39.104-87.341-87.341-87.341v0z"}],
                    path![attrs!{At::D => "M87.944 0c-0.179-0.001-0.391-0.002-0.602-0.002-48.237 0-87.341 39.104-87.341 87.341 0 0.001 0 0.002 0 0.002l-0-0v849.318c-0.096 1.532-0.151 3.323-0.151 5.127 0 48.237 39.104 87.341 87.341 87.341s87.341-39.104 87.341-87.341c0-1.804-0.055-3.594-0.162-5.371l0.012 0.244v-849.318c0-0.090 0.001-0.197 0.001-0.303 0-47.859-38.627-86.697-86.406-87.038l-0.032-0z"}],
                    path![attrs!{At::D => "M380.988 171.369c-48.002 0.171-86.869 39.038-87.040 87.024l-0 0.016v678.249c-0.096 1.532-0.151 3.323-0.151 5.127 0 48.237 39.104 87.341 87.341 87.341s87.341-39.104 87.341-87.341c0-1.804-0.055-3.594-0.162-5.371l0.012 0.244v-678.249c-0.171-48.108-39.209-87.040-87.341-87.040-0 0-0 0-0.001 0l0-0z"}],
                ])
            )
        },
        {
            let active = false;
            vertical_nav_button(
                "Settings", 
                active,
                RootUrls::new(root_base_url).settings(),  
                false, 
                vertical_nav_icon("ic_settings", "0 0 1043 1024", active, vec![
                    path![attrs!{At::D => "M791.492 901.421c-0.137 1.886-0.214 4.085-0.214 6.303 0 14.689 3.414 28.58 9.492 40.924l-0.242-0.544c1.442 2.027 2.306 4.553 2.306 7.281 0 5.548-3.572 10.262-8.542 11.967l-0.089 0.027c-37.735 21.585-81.411 40.158-127.33 53.451l-4.284 1.062c-2.114 1.002-4.593 1.587-7.209 1.587-7.903 0-14.559-5.341-16.556-12.61l-0.028-0.12c-20.88-43.535-64.606-73.060-115.229-73.060-26.819 0-51.703 8.287-72.23 22.44l0.428-0.279c-19.628 13.227-34.808 31.704-43.688 53.426l-0.284 0.786c-3.614 8.734-7.529 11.746-17.769 9.035-51.834-13.272-97.233-31.525-139.449-54.835l3.016 1.527c-14.758-7.831-8.734-16.866-5.12-26.805 4.846-12.398 7.654-26.752 7.654-41.762 0-32.050-12.804-61.11-33.576-82.344l0.021 0.021c-22.874-25.484-55.92-41.441-92.693-41.441-10.83 0-21.336 1.384-31.352 3.985l0.864-0.191h-5.722c-30.118 9.336-30.118 9.035-44.273-18.372-17.236-31.193-32.683-67.512-44.377-105.477l-1.101-4.152c-3.915-12.348-1.807-18.673 11.445-24.094 45.171-18.059 76.501-61.451 76.501-112.16 0-0.275-0.001-0.549-0.003-0.823l0 0.042c-0.157-51.84-32.003-96.203-77.176-114.748l-0.829-0.301c-13.553-4.819-15.962-10.842-12.047-23.793 13.962-48.504 31.914-90.674 54.24-130.036l-1.534 2.94c6.024-10.541 11.746-12.649 23.793-7.831 14.648 6.459 31.727 10.219 49.685 10.219 35.285 0 67.18-14.517 90.038-37.904l0.023-0.024c21.532-21.755 34.835-51.691 34.835-84.733 0-19.022-4.409-37.015-12.26-53.011l0.314 0.709c-4.216-9.638-3.012-15.059 6.024-20.48 39.702-23.013 85.609-42.536 133.977-56.195l4.263-1.029c13.252-3.614 14.758 5.12 18.372 13.252 16.261 41.325 53.282 71.221 97.87 77.036l0.614 0.065c6.241 1.121 13.425 1.762 20.759 1.762 40.852 0 77.059-19.886 99.469-50.507l0.242-0.347c7.452-9.232 13.404-20.047 17.264-31.809l0.204-0.718c3.012-8.433 8.132-9.939 16.264-8.132 52.584 13.65 98.681 32.83 141.232 57.456l-2.691-1.437c9.336 5.12 8.433 11.144 4.819 19.576-6.604 14.774-10.451 32.016-10.451 50.158 0 69.362 56.229 125.591 125.591 125.591 18.623 0 36.299-4.053 52.195-11.326l-0.784 0.321c10.24-4.518 15.962-3.012 21.384 6.927 22.212 37.657 40.917 81.17 53.87 127.095l0.944 3.916c2.711 10.24 0 15.36-10.24 19.878-46.208 16.823-78.61 60.371-78.61 111.487 0 0.299 0.001 0.599 0.003 0.898l-0-0.046c-0.106 1.871-0.166 4.060-0.166 6.264 0 49.766 30.792 92.34 74.362 109.71l0.797 0.28c12.951 6.024 16.264 11.746 12.047 25.6-14.446 47.781-32.562 89.199-54.858 127.907l1.55-2.918c-5.421 10.24-10.842 12.348-22.287 8.132-14.209-5.966-30.724-9.432-48.048-9.432-45.354 0-85.159 23.756-107.651 59.503l-0.31 0.527c-11.029 16.816-17.591 37.422-17.591 59.561 0 1.826 0.045 3.642 0.133 5.446l-0.010-0.254zM520.433 711.68c109.44-1.529 197.571-90.604 197.571-200.264 0-110.613-89.669-200.282-200.282-200.282s-200.282 89.669-200.282 200.282c0 0.205 0 0.411 0.001 0.616l-0-0.032c0.498 110.402 90.11 199.707 200.582 199.707 1.166 0 2.329-0.010 3.49-0.030l-0.175 0.002z"}],
                ])
            )
        },
        {
            let active = page_id == PageId::Addons;
            vertical_nav_button(
                "Addons", 
                active,
                RootUrls::new(root_base_url).addons_urls().root(), 
                false, 
                vertical_nav_icon("ic_addons", "0 0 1043 1024", active, vec![
                    path![attrs!{At::D => "M145.468 679.454c-40.056-39.454-80.715-78.908-120.471-118.664-33.431-33.129-33.129-60.235 0-90.353l132.216-129.807c5.693-5.938 12.009-11.201 18.865-15.709l0.411-0.253c23.492-15.059 41.864-7.529 48.188 18.974 0 7.228 2.711 14.758 3.614 22.287 3.801 47.788 37.399 86.785 82.050 98.612l0.773 0.174c10.296 3.123 22.128 4.92 34.381 4.92 36.485 0 69.247-15.94 91.702-41.236l0.11-0.126c24.858-21.654 40.48-53.361 40.48-88.718 0-13.746-2.361-26.941-6.701-39.201l0.254 0.822c-14.354-43.689-53.204-75.339-99.907-78.885l-0.385-0.023c-18.372-2.409-41.562 0-48.188-23.492s11.445-34.635 24.998-47.887q65.054-62.946 130.409-126.795c32.527-31.925 60.235-32.226 90.353 0 40.659 39.153 80.715 78.908 120.471 118.362 8.348 8.594 17.297 16.493 26.82 23.671l0.587 0.424c8.609 7.946 20.158 12.819 32.846 12.819 24.823 0 45.29-18.653 48.148-42.707l0.022-0.229c3.012-13.252 4.518-26.805 8.734-39.755 12.103-42.212 50.358-72.582 95.705-72.582 3.844 0 7.637 0.218 11.368 0.643l-0.456-0.042c54.982 6.832 98.119 49.867 105.048 104.211l0.062 0.598c0.139 1.948 0.218 4.221 0.218 6.512 0 45.084-30.574 83.026-72.118 94.226l-0.683 0.157c-12.348 3.915-25.299 5.722-37.948 8.433-45.779 9.638-60.235 46.984-30.118 82.824 15.265 17.569 30.806 33.587 47.177 48.718l0.409 0.373c31.925 31.925 64.452 62.946 96.075 94.871 13.698 9.715 22.53 25.511 22.53 43.369s-8.832 33.655-22.366 43.259l-0.164 0.111c-45.176 45.176-90.353 90.353-137.035 134.325-5.672 5.996-12.106 11.184-19.169 15.434l-0.408 0.227c-4.663 3.903-10.725 6.273-17.341 6.273-13.891 0-25.341-10.449-26.92-23.915l-0.012-0.127c-2.019-7.447-3.714-16.45-4.742-25.655l-0.077-0.848c-4.119-47.717-38.088-86.476-82.967-97.721l-0.76-0.161c-9.584-2.63-20.589-4.141-31.947-4.141-39.149 0-74.105 17.956-97.080 46.081l-0.178 0.225c-21.801 21.801-35.285 51.918-35.285 85.185 0 1.182 0.017 2.36 0.051 3.533l-0.004-0.172c1.534 53.671 40.587 97.786 91.776 107.115l0.685 0.104c12.649 2.409 25.901 3.313 38.249 6.626 22.588 6.325 30.118 21.685 18.372 41.864-4.976 8.015-10.653 14.937-17.116 21.035l-0.051 0.047c-44.875 44.574-90.353 90.353-135.228 133.12-10.241 14.067-26.653 23.106-45.176 23.106s-34.935-9.039-45.066-22.946l-0.111-0.159c-40.659-38.852-80.414-78.908-120.471-118.362z"}],
                ])
            )
        },
    ]
}

#[view]
fn vertical_nav_button(title: &str, active: bool, url: Url, margin_top: bool, icon: Node<Msg>) -> Node<Msg> {
    a![
        C!["nav-tab-button", "nav-tab-button-container"],
        attrs!{
            At::TabIndex => -1,
            At::Title => title,
            At::Href => url,
        },
        IF!(margin_top => s().margin_top(rem(1))),
        s()
            .height(global::VERTICAL_NAV_BAR_SIZE)
            .width(global::VERTICAL_NAV_BAR_SIZE)
            .align_items(CssAlignItems::Center)
            .background_color(if active { Color::SecondaryVariant1Dark5 } else { Color::BackgroundDark1 })
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .justify_content(CssJustifyContent::Center)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(if active { Color::SecondaryVariant1Dark4 } else { Color::BackgroundLight2 }),
        icon,
        vertical_nav_label(title, active),
    ]
}

#[view]
fn vertical_nav_icon(icon: &str, view_box: &str, active: bool, paths: Vec<Node<Msg>>) -> Node<Msg> {
    svg![
        C!["icon"],
        attrs!{
            At::from("icon") => icon,
            At::ViewBox => view_box,
        },
        s()
            .overflow(CssOverflow::Visible)
            .fill(if active { Color::SurfaceLight5_90 } else { Color::SecondaryLight5_90 })
            .flex(CssFlex::None)
            .height(rem(1.7))
            .margin_bottom(rem(0.5))
            .width(rem(1.7)),
        paths,
    ]
}

#[view]
fn vertical_nav_label(title: &str, active: bool) -> Node<Msg> {
    div![
        C!["label"],
        s()
            .color(if active { Color::SurfaceLight5_90 } else { Color::SecondaryVariant1_90 })
            .flex(CssFlex::None)
            .font_size(rem(0.9))
            .font_weight("500")
            .letter_spacing(rem(0.01))
            .max_height(em(2.4))
            .padding("0 0.2rem")
            .text_align(CssTextAlign::Center),
        title,
    ]
}

#[view]
fn nav_content_container(page_content: Node<Msg>) -> Node<Msg> {
    div![
        C!["nav-content-container"],
        s()
            .bottom("0")
            .left(global::VERTICAL_NAV_BAR_SIZE)
            .position(CssPosition::Absolute)
            .right("0")
            .top(global::HORIZONTAL_NAV_BAR_SIZE)
            .z_index("0"),
        page_content,
    ]
}
