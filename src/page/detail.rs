use crate::{PageId, Actions, Context};
use crate::styles::global;
use seed::{prelude::*, *};
use std::rc::Rc;
use std::collections::HashMap;
use stremio_core::runtime::msg::{Msg as CoreMsg, Action, ActionLoad, ActionCtx};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use crate::styles::{self, themes::Color};
use stremio_core::models::meta_details::Selected as MetaDetailsSelected;
use stremio_core::models::common::{ResourceLoadable, Loadable};
use stremio_core::types::addon::ResourcePath;
use stremio_core::types::resource::{MetaItem, Link, MetaItemPreview, Video};
use stremio_core::types::library::LibraryItem;
use seed_hooks::{*, topo::nested as view};

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!").unwrap(); })
}

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let base_url = url.to_hash_base_url();

    let type_name = url.next_hash_path_part()?.to_owned();
    let id = url.next_hash_path_part()?.to_owned();
    let video_id = url.next_hash_path_part();

    let selected_meta_details = MetaDetailsSelected {
        meta_path: ResourcePath::without_extra("meta", &type_name, &id),
        stream_path: video_id.map(|video_id| ResourcePath::without_extra("stream", &type_name, video_id)),
    };
    orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::MetaDetails(selected_meta_details)
    )))));

    model.get_or_insert_with(|| Model {
        base_url,
        search_query: String::new(),
    });
    Some(PageId::Detail)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    search_query: String,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn without_video_id(self, type_name: &str, id: &str) -> Url {
        self.base_url()
            .add_hash_path_part(type_name)
            .add_hash_path_part(id)
    }
    pub fn with_video_id(self, type_name: &str, id: &str, video_id: &str) -> Url {
        self.base_url()
            .add_hash_path_part(type_name)
            .add_hash_path_part(id)
            .add_hash_path_part(video_id)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    AddToLibrary(MetaItem),
    RemoveFromLibrary(String),
    SearchQueryChanged(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddToLibrary(meta_item) => {
            let item = MetaItemPreview {
                id: meta_item.id,
                r#type: meta_item.r#type,
                name: meta_item.name,
                poster: meta_item.poster,
                logo: meta_item.logo,
                description: meta_item.description,
                release_info: meta_item.release_info,
                runtime: meta_item.runtime,
                released: meta_item.released,
                poster_shape: meta_item.poster_shape,
                trailer_streams: meta_item.trailer_streams,
                behavior_hints: meta_item.behavior_hints,
            };
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::AddToLibrary(item)
            )))));
        }
        Msg::RemoveFromLibrary(id) => {
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::RemoveFromLibrary(id)
            )))));
        }
        Msg::SearchQueryChanged(query) => {
            model.search_query = query
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context) -> Node<Msg> {
    // let streams = &context.core_model.meta_details.streams;
    // log!(streams.len());

    let meta_items = &context.core_model.meta_details.meta_items;
    let library = &context.core_model.ctx.library.items;
    div![
        C!["metadetails-container",],
        s()
            .background_color(Color::BackgroundDark2)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .height(pc(100))
            .width(pc(100)),
        nav_bar(meta_items),
        div![
            C!["metadetails-content"],
            s()
                .align_self(CssAlignSelf::Stretch)
                .display(CssDisplay::Flex)
                .flex("1")
                .flex_direction(CssFlexDirection::Row)
                .position(CssPosition::Relative)
                .z_index("0"),
            background_image_layer(meta_items),
            meta_preview(meta_items, library),
            div![
                C!["spacing"],
                s()
                    .flex("1"),
            ],
            side_bar(meta_items, &model.search_query, &model.base_url),
        ]
    ]
}

#[view]
fn side_bar(meta_items: &[ResourceLoadable<MetaItem>], search_query: &str, base_url: &Url) -> Option<Node<Msg>> {
    if let Loadable::Ready(meta_item) = &meta_items.first()?.content {
        match meta_item.r#type.as_str() {
            "series" | "other" => return Some(videos_list(&meta_item, search_query, base_url)),
            "movie" => return Some(streams_list(meta_item)),
            _ => log!("unknown meta item type"),
        }
    }
    None
}

#[view]
fn videos_list(meta_item: &MetaItem, search_query: &str, base_url: &Url) -> Node<Msg> {
    div![
        C!["videos-list", "videos-list-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .background_color(hsla(0, 0, 0, 0.7))
            .flex("0 0 26.5rem")
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column),
        search_bar(search_query),
        videos_container(meta_item, search_query, base_url),
    ]
}

#[view]
fn videos_container(meta_item: &MetaItem, search_query: &str, base_url: &Url) -> Node<Msg> {
    div![
        C!["videos-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto),
        meta_item
            .videos
            .iter()
            .filter(|video| video.title.contains(search_query))
            .map(|video| video_container(video, meta_item, base_url)),
    ]
}

#[view]
fn video_container(video: &Video, meta_item: &MetaItem, base_url: &Url) -> Node<Msg> {
    a![
        C!["video-container", "button-container"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .flex_wrap(CssFlexWrap::Wrap)
            .padding("0.5rem 1rem")
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::Background),
        attrs!{
            At::Title => &video.title,
            At::TabIndex => 0,
            At::Href => Urls::new(base_url).with_video_id(&meta_item.r#type, &meta_item.id, &video.id),
        },
        div![
            C!["info-container"],
            s()
                .height(rem(3))
                .align_self(CssAlignSelf::Stretch)
                .display(CssDisplay::Flex)
                .flex("1")
                .flex_direction(CssFlexDirection::Column)
                .justify_content(CssJustifyContent::SpaceBetween)
                .margin("0.5rem 1rem"),
            div![
                C!["title-container"],
                s()
                    .max_height(em(1.2))
                    .color(Color::SurfaceLight5_90),
                &video.title,
            ],
            div![
                C!["flex-row-container"],
                s()
                    .align_items(CssAlignItems::Center)
                    .display(CssDisplay::Flex)
                    .flex_direction(CssFlexDirection::Row)
                    .justify_content(CssJustifyContent::FlexEnd),
                div![
                    C!["released-container"],
                    s()
                        .color(Color::SurfaceDark5_90)
                        .flex("1")
                        .font_size(rem(0.8))
                        .font_weight("500")
                        .margin_right(rem(0.5))
                        .text_overflow("ellipsis")
                        .text_transform(CssTextTransform::Uppercase)
                        .white_space(CssWhiteSpace::NoWrap),
                    video.released.as_ref().map(|released| {
                        // 15 Apr 21
                        released.format("%e %b %y").to_string()
                    }),
                ],
                div![
                    C!["upcoming-watched-container"],
                    s()
                        .display(CssDisplay::Flex)
                        .flex("0 1 auto")
                        .flex_direction(CssFlexDirection::Row),
                ]
            ]
        ]
    ]
}

#[view]
fn search_bar(search_query: &str) -> Node<Msg> {
    label![
        C!["search-bar", "search-bar-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex(CssFlex::None)
            .margin("1rem 1.5rem 1rem")
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .border(format!("{} solid transparent", global::FOCUS_OUTLINE_SIZE).as_str())
            .border_radius(rem(3.5))
            .cursor(CssCursor::Text)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(rem(3.5))
            .padding("0 1rem"),
        s()
            .focus_within()
            .background_color(Color::BackgroundLight1)
            .border(format!("{} solid hsl(0,0%,100%)", global::FOCUS_OUTLINE_SIZE).as_str()),
        s()
            .hover()
            .background_color(Color::BackgroundLight1),
        attrs!{
            At::Title => "Search videos",
        },
        input![
            C!["search-input", "text-input"],
            s()
                .color(Color::SurfaceLight5)
                .flex("1")
                .font_size(rem(1.1))
                .margin_right(rem(1))
                .user_select("text"),
            s()
                .style_other("::placeholder")
                .color(Color::SecondaryVariant1Light1_90)
                .max_height(em(1.2))
                .opacity("1"),
            attrs!{
                At::from("autocorrect") => "off",
                At::from("autocapitalize") => "none",
                At::AutoComplete => "off",
                At::SpellCheck => "false",
                At::TabIndex => 0,
                At::Type => "text",
                At::Placeholder => "Search videos",
                At::Value => search_query,
            },
            input_ev(Ev::Input, Msg::SearchQueryChanged),
        ],
        search_bar_icon()
    ]
}

#[view]
fn search_bar_icon() -> Node<Msg> {
    svg![
        C!["icon"],
        s()
            .fill(Color::SecondaryVariant1_90)
            .flex(CssFlex::None)
            .height(rem(1.5))
            .width(rem(1.5)),
        attrs!{
            At::ViewBox => "0 0 1025 1024",
            At::from("icon") => "ic_search",
        },
        path![
            attrs!{
                At::D => "M1001.713 879.736c-48.791-50.899-162.334-163.84-214.438-216.546 43.772-66.969 69.909-148.918 70.174-236.956l0-0.070c-1.877-235.432-193.166-425.561-428.862-425.561-236.861 0-428.875 192.014-428.875 428.875 0 236.539 191.492 428.353 427.909 428.874l0.050 0c1.551 0.021 3.382 0.033 5.216 0.033 85.536 0 165.055-25.764 231.219-69.956l-1.518 0.954 201.487 204.499c16.379 18.259 39.94 29.789 66.201 30.117l0.058 0.001c2.034 0.171 4.401 0.269 6.791 0.269 35.32 0 65.657-21.333 78.83-51.816l0.214-0.556c5.589-10.528 8.87-23.018 8.87-36.275 0-21.857-8.921-41.631-23.32-55.878l-0.007-0.007zM429.478 730.654c-0.004 0-0.008 0-0.012 0-166.335 0-301.176-134.841-301.176-301.176 0-0.953 0.004-1.905 0.013-2.856l-0.001 0.146c0.599-165.882 135.211-300.124 301.176-300.124 166.336 0 301.178 134.842 301.178 301.178 0 0.371-0.001 0.741-0.002 1.111l0-0.057c0 0.179 0.001 0.391 0.001 0.603 0 166.335-134.841 301.176-301.176 301.176-0.106 0-0.212-0-0.318-0l0.016 0z",
            }
        ]
    ]
}

#[view]
fn streams_list(meta_item: &MetaItem) -> Node<Msg> {
    div![
        C!["streams-list", "streams-list-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .background_color(hsla(0, 0, 0, 0.7))
            .flex("0 0 26.5rem")
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column),
    ]
}

#[view]
fn nav_bar(meta_items: &[ResourceLoadable<MetaItem>]) -> Node<Msg> {
    div![
        C!["nav-bar", "horizontal-nav-bar-container",],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex(CssFlex::None)
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(styles::global::HORIZONTAL_NAV_BAR_SIZE)
            .overflow(CssOverflow::Visible)
            .padding_right(rem(1)),
        back_button(),
        nav_bar_title(&meta_items),
    ]
}

#[view]
fn back_button() -> Node<Msg> {
    div![
        C!["button-container", "back-button-container", "button-container"],
        s()
            .height(styles::global::HORIZONTAL_NAV_BAR_SIZE)
            .width(styles::global::VERTICAL_NAV_BAR_SIZE)
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .justify_content(CssJustifyContent::Center)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        attrs!{
            At::TabIndex => -1,
        },
        ev(Ev::Click, |_| Url::go_back(1)),
        svg![
            C!["icon"],
            s()
                .fill(Color::SecondaryVariant2Light1_90)
                .flex(CssFlex::None)
                .height(rem(1.7))
                .width(rem(1.7)),
            attrs! {
                At::ViewBox => "0 0 607 1024",
                "icon" => "ic_back_ios",
            },
            path![attrs! {
                At::D => "M607.473 926.419l-412.009-414.419 412.009-414.419-97.28-97.581-510.193 512 510.193 512z"
            }]
        ]
    ]
}

#[view]
fn nav_bar_title(meta_items: &[ResourceLoadable<MetaItem>]) -> Option<Node<Msg>> {
    if let Loadable::Ready(meta_item) = &meta_items.first()?.content {
        return Some(h2![
            C!["title"], 
            s()
                .color(Color::SecondaryVariant2Light1_90)
                .flex("4 0 0")
                .font_size(rem(1.2))
                .font_style(CssFontStyle::Normal)
                .font_weight("500")
                .letter_spacing(rem(0.01))
                .padding("0 1rem")
                .text_overflow("ellipsis")
                .white_space(CssWhiteSpace::NoWrap),
            &meta_item.name,
        ])
    }
    None
}

#[view]
fn background_image_layer(meta_items: &[ResourceLoadable<MetaItem>]) -> Node<Msg> {
    div![
        C!["background-image-layer"],
        s()
            .bottom("0")
            .position(CssPosition::Absolute)
            .top("0")
            .right("0")
            .left("0")
            .z_index("-1"),
        meta_items.first().and_then(|meta_item| {
            if let Loadable::Ready(meta_item) = &meta_item.content {
                meta_item.background.as_ref().map(|background| {
                    img![
                        C!["background-image"],
                        s()
                            .display(CssDisplay::Block)
                            .height(pc(100))
                            .raw("object-fit: cover;")
                            .raw("object-position: top left;")
                            .opacity("0.9")
                            .width(pc(100)),
                        attrs! {
                            At::Src => background,
                            At::Alt => " ",
                        }
                    ]
                })
            } else {
                None
            }
        }),
        div![
            C!["background-overlay"],
            s()
                .background_color(Color::BackgroundDark2_70)
                .bottom("0")
                .position(CssPosition::Absolute)
                .top("0")
                .right("0")
                .left("0")
                .z_index("1"),
        ]
    ]
}

#[view]
fn meta_preview(meta_items: &[ResourceLoadable<MetaItem>], library: &HashMap<String, LibraryItem>) -> Option<Node<Msg>> {
    if let Loadable::Ready(meta_item) = &meta_items.first()?.content {
        return Some(div![
            C!["meta-preview", "meta-preview-container",],
            s()
                .align_self(CssAlignSelf::Stretch)
                .flex("0 1 40rem")
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Column)
                .position(CssPosition::Relative)
                .z_index("0"),
            meta_info(meta_item),
            action_buttons(meta_item, library),
        ])
    }
    None
}

#[view]
fn meta_info(meta_item: &MetaItem) -> Node<Msg> {
    div![
        C!["meta-info-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 2rem"),
        meta_info_logo(meta_item),
        meta_info_runtime_release(meta_item),
        div![
            C!["name-container"],
            s()
                .color(Color::SurfaceLight5_90)
                .font_size(rem(1.7))
                .margin_top(rem(1)),
            &meta_item.name,
        ],
        meta_item.description.as_ref().map(|description| {
            div![
                C!["description-container"],
                s()
                    .color(Color::SurfaceLight5_90)
                    .font_size(rem(1.1))
                    .line_height(em(1.5))
                    .margin_top(rem(1))
                    .max_height(em(6)),
                description,
            ]
        }),
        meta_links(meta_item),
    ]
}

#[view]
fn meta_info_logo(meta_item: &MetaItem) -> Option<Node<Msg>> {
    meta_item.logo.as_ref().map(|logo| {
        img![
            C!["logo"],
            s()
                .raw("object-fit: contain;")
                .raw("object-position: center;")
                .display(CssDisplay::Block)
                .height(rem(8))
                .margin("2rem 0")
                .max_width(pc(100)),
            attrs!{
                At::Src => logo,
                At::Alt => " ",
            }
        ]
    })
}

#[view]
fn meta_info_runtime_release(meta_item: &MetaItem) -> Node<Msg> {
    div![
        C!["runtime-release-info-container"],
        s()
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .flex_wrap(CssFlexWrap::Wrap)
            .margin_top(rem(1)),
        meta_item.runtime.as_ref().map(|runtime| {
            div![
                C!["runtime-label"],
                s()
                    .color(Color::SurfaceLight5_90)
                    .flex("0 1 auto")
                    .font_size(rem(1.4))
                    .margin_bottom(rem(0.5))
                    .margin_right(rem(2)),
                runtime,
            ]
        }),
        meta_item.release_info.as_ref().map(|release_info| {
            div![
                C!["release-info-label"],
                s()
                    .color(Color::SurfaceLight5_90)
                    .flex("0 1 auto")
                    .font_size(rem(1.4))
                    .margin_bottom(rem(0.5))
                    .margin_right(rem(2)),
                release_info,
            ]
        }),
        imdb_pill(meta_item),
    ]
}

#[view]
fn imdb_pill(meta_item: &MetaItem) -> Option<Node<Msg>> {
    meta_item.links.iter().find(|link| link.category == "imdb").map(|link| {
        a![
            C!["imdb-button-container", "button-container"],
            s()
                .align_items(CssAlignItems::Center)
                .background_color(Color::SurfaceLight5_20)
                .border(format!("{} solid transparent", styles::global::FOCUS_OUTLINE_SIZE).as_str())
                .border_radius(rem(2.5))
                .display(CssDisplay::Flex)
                .flex("0 1 auto")
                .flex_direction(CssFlexDirection::Row)
                .margin_bottom(rem(0.5))
                .padding("0.3rem 1rem")
                .cursor(CssCursor::Pointer),
            s()
                .hover()
                .background_color(Color::SurfaceLight5_30),
            attrs!{
                At::Href => &link.url,
                At::TabIndex => 0,
                At::Title => &link.name,
                At::Target => "_blank",
            },
            svg![
                C!["icon"],
                s()
                    .fill(Color::Surface90)
                    .flex(CssFlex::None)
                    .height(rem(1.1))
                    .margin_right(rem(1))
                    .width(rem(3)),
                attrs!{
                    At::ViewBox => "0 0 2874 1024",
                    At::from("icon") => "ic_imdbnoframe",
                },
                path![
                    attrs!{
                        At::D => "M0 0h197.873v1013.459h-197.873v-1013.459z",
                    }
                ],
                path![
                    attrs!{
                        At::D => "M794.805 683.068l-193.054-683.068h-251.784v1013.459h173.478v-706.259l187.031 593.318h168.659l186.729-606.569v719.511h173.478v-1013.459h-251.482l-193.054 683.068z",
                    }
                ],
                path![
                    attrs!{
                        At::D => "M1740.8 0h-335.511v1013.459h334.607q190.946 0 295.755-95.172t104.508-268.047v-287.021q0-172.875-104.207-268.047t-295.153-95.172zM1945.6 655.962c0.14 2.502 0.219 5.431 0.219 8.378 0 47.010-20.245 89.292-52.493 118.601l-0.131 0.117c-36.827 28.45-83.646 45.602-134.47 45.602-4.823 0-9.61-0.154-14.356-0.459l0.647 0.033h-148.781v-644.216h148.48c4.099-0.271 8.886-0.425 13.708-0.425 50.825 0 97.643 17.152 134.983 45.983l-0.513-0.381c32.41 29.579 52.664 72.002 52.664 119.156 0 3.217-0.094 6.413-0.28 9.583l0.021-0.438z",
                    }
                ],
                path![
                    attrs!{
                        At::D => "M2630.174 279.492c-0.356-0.003-0.778-0.004-1.2-0.004-39.282 0-75.526 13.011-104.656 34.96l0.444-0.32c-25.55 19.818-46.465 44.29-61.741 72.282l-0.602 1.205v-387.614h-191.247v1013.459h191.247v-95.473c26.883 62.914 88.237 106.202 159.701 106.202 2.833 0 5.65-0.068 8.45-0.203l-0.395 0.015c3.585 0.214 7.778 0.336 11.999 0.336 68.311 0 129.151-31.986 168.359-81.798l0.348-0.458q63.548-81.92 63.548-231.002v-115.652q0-150.588-63.548-233.412c-39.49-50.608-100.489-82.842-169.016-82.842-4.112 0-8.197 0.116-12.252 0.345l0.562-0.025zM2682.579 711.078c0.154 2.574 0.242 5.584 0.242 8.615 0 34.54-11.401 66.418-30.645 92.075l0.285-0.398c-18.89 21.912-46.683 35.699-77.697 35.699-2.015 0-4.017-0.058-6.004-0.173l0.275 0.013c-1.239 0.047-2.695 0.075-4.156 0.075-19.738 0-38.324-4.932-54.593-13.63l0.623 0.304c-15.974-8.736-28.61-21.857-36.517-37.745l-0.227-0.504c-7.83-16.554-12.402-35.963-12.402-56.439 0-1.335 0.019-2.665 0.058-3.991l-0.004 0.195v-162.936c-0.047-1.369-0.074-2.977-0.074-4.592 0-21.028 4.576-40.988 12.786-58.938l-0.363 0.886c8.012-16.829 20.666-30.355 36.307-39.227l0.436-0.228c15.781-8.571 34.555-13.609 54.506-13.609 1.273 0 2.542 0.021 3.805 0.061l-0.184-0.005c1.532-0.082 3.325-0.129 5.13-0.129 31.373 0 59.44 14.15 78.169 36.416l0.127 0.155c18.981 26.070 30.367 58.731 30.367 94.051 0 3.149-0.091 6.277-0.269 9.381l0.020-0.43z",
                    }
                ],
            ],
            div![
                C!["label"],
                s()
                    .color(Color::SurfaceLight5_90)
                    .flex("0 1 auto")
                    .font_size(rem(1.6))
                    .font_weight("500")
                    .max_height(em(1.2)),
                &link.name,
            ]
        ]
    })
}

#[view]
fn meta_links(meta_item: &MetaItem) -> Vec<Node<Msg>> {
    let mut genres = Vec::new(); 
    let mut cast = Vec::new(); 
    let mut writers = Vec::new(); 
    let mut directors = Vec::new();

    for link in &meta_item.links {
        match link.category.to_lowercase().as_str() {
            "genres" => genres.push(link),
            "cast" => cast.push(link),
            "writers" => writers.push(link),
            "directors" => directors.push(link),
            _ => (),
        }
    }
    
    let mut nodes = Vec::new();
    
    if !genres.is_empty() {
        nodes.push(meta_link_group("GENRES", &genres));
    } 
    if !cast.is_empty() {
        nodes.push(meta_link_group("CAST", &cast));
    } 
    if !writers.is_empty() {
        nodes.push(meta_link_group("WRITERS", &writers));
    } 
    if !directors.is_empty() {
        nodes.push(meta_link_group("DIRECTORS", &directors));
    } 
    nodes
}

#[view]
fn meta_link_group(title: &str, links: &[&Link]) -> Node<Msg> {
    div![
        C!["meta-links", "meta-links-container"],
        s()
            .margin_top(rem(1)),
        div![
            C!["label-container"],
            s()
                .color(Color::SurfaceDark3_90)
                .font_weight("500")
                .margin_bottom(rem(0.2)),
            title,
        ],
        div![
            C!["links-container"],
            s()
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .flex_wrap(CssFlexWrap::Wrap),
            links.iter().map(|link| meta_link(link)),
        ]
    ]
}

#[view]
fn meta_link(link: &Link) -> Node<Msg> {
    a![
        C!["link-container", "button-container"],
        s()
            .background_color(Color::SurfaceLight5_20)
            .border(format!("{} solid transparent", styles::global::FOCUS_OUTLINE_SIZE).as_str())
            .border_radius(rem(2))
            .color(Color::SurfaceLight2_90)
            .flex_basis(CssFlexBasis::Auto)
            .flex_grow("0")
            .flex_shrink("0")
            .margin_bottom(rem(0.2))
            .margin_right(rem(0.5))
            .padding("0.3rem 0.5rem")
            .text_overflow("ellipsis")
            .white_space(CssWhiteSpace::NoWrap)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::SurfaceLight5_30),
        attrs!{
            At::TabIndex => 0,
            At::Title => &link.name,
            At::Href => &link.url,
        },
        &link.name,
    ]
}

#[view]
fn action_buttons(meta_item: &MetaItem, library: &HashMap<String, LibraryItem>) -> Node<Msg> {
    div![
        C!["action-buttons-container",],
        s()
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .flex_wrap(CssFlexWrap::Wrap)
            .max_height(rem(10))
            .padding("0 2rem"),
        if library.get(&meta_item.id).map(|library_item| not(library_item.removed)).unwrap_or_default() {
            let id = meta_item.id.to_owned();
            action_button(
                "Remove from Library", "0 0 1264 1024", "ic_removelib", 
                remove_from_library_paths(), 
                ev(Ev::Click, move |_| Msg::RemoveFromLibrary(id))
            )
        } else {
            let meta_item = meta_item.to_owned();
            action_button(
                "Add to Library", "0 0 1264 1024", "ic_addlib", 
                add_to_library_paths(),
                ev(Ev::Click, move |_| Msg::AddToLibrary(meta_item))
            )
        },
        IF!(not(meta_item.trailer_streams.is_empty()) => {
            action_button("Trailer", "0 0 840 1024", "ic_movies", trailer_paths(), None)
        }),
        action_button("Share", "0 0 1024 1024", "ic_share", share_paths(), None),
    ]
}

fn add_to_library_paths() -> Vec<Node<Msg>> {
    vec![
        path![attrs! {
            At::D => "M78.306 0c-43.178 0.17-78.135 35.127-78.306 78.29l-0 0.016v764.988c2.636 41.27 36.754 73.744 78.456 73.744s75.82-32.474 78.445-73.514l0.012-0.23v-764.988c-0.171-43.284-35.299-78.306-78.606-78.306-0 0-0 0-0.001 0l0 0z"
        }],
        path![attrs! {
            At::D => "M341.835 153.901c-43.178 0.17-78.135 35.127-78.306 78.29l-0 0.016v611.087c0 43.663 35.396 79.059 79.059 79.059s79.059-35.396 79.059-79.059v0-611.087c-0.166-43.288-35.296-78.315-78.607-78.315-0.424 0-0.847 0.003-1.269 0.010l0.064-0.001z"
        }],
        path![attrs! {
            At::D => "M963.765 421.647c-166.335 0-301.176 134.841-301.176 301.176s134.841 301.176 301.176 301.176c166.335 0 301.176-134.841 301.176-301.176v0c0-166.335-134.841-301.176-301.176-301.176v0zM1156.518 768.602h-148.179v147.275h-90.353v-148.179h-147.878v-90.353h147.275v-147.878h90.353v147.275h147.275z"
        }],
        path![attrs! {
            At::D => "M683.972 465.016v-386.711c-2.636-41.27-36.754-73.744-78.456-73.744s-75.82 32.474-78.445 73.514l-0.012 0.23v764.988c-0 0-0 0-0 0.001 0 43.247 35.059 78.306 78.306 78.306 0.106 0 0.212-0 0.318-0.001l-0.016 0c0.068 0 0.147 0 0.227 0 10.82 0 21.097-2.329 30.355-6.513l-0.465 0.188c-32.753-54.79-52.119-120.857-52.119-191.447 0-99.528 38.499-190.064 101.417-257.529l-0.206 0.223z"
        }],
        path![attrs! {
            At::D => "M817.092 371.351c42.987-18.759 93.047-29.807 145.652-30.117l0.117-0.001h8.433l-60.235-262.325c-8.294-35.054-39.322-60.736-76.348-60.736-43.274 0-78.355 35.081-78.355 78.355 0 6.248 0.731 12.325 2.113 18.151l-0.106-0.532z"
        }],
    ]
}

fn remove_from_library_paths() -> Vec<Node<Msg>> {
    vec![
        path![attrs! {
            At::D => "M78.306 0c-43.178 0.17-78.135 35.127-78.306 78.29l-0 0.016v764.988c2.636 41.27 36.754 73.744 78.456 73.744s75.82-32.474 78.445-73.514l0.012-0.23v-764.988c-0.171-43.284-35.299-78.306-78.606-78.306-0 0-0 0-0.001 0l0 0z"
        }],
        path![attrs! {
            At::D => "M341.835 153.901c-43.178 0.17-78.135 35.127-78.306 78.29l-0 0.016v611.087c0 43.663 35.396 79.059 79.059 79.059s79.059-35.396 79.059-79.059v0-611.087c-0.166-43.288-35.296-78.315-78.607-78.315-0.424 0-0.847 0.003-1.269 0.010l0.064-0.001z"
        }],
        path![attrs! {
            At::D => "M963.765 421.647c-166.335 0-301.176 134.841-301.176 301.176s134.841 301.176 301.176 301.176c166.335 0 301.176-134.841 301.176-301.176v0c0-166.335-134.841-301.176-301.176-301.176v0zM1156.518 768.602h-386.409v-90.353h385.506z"
        }],
        path![attrs! {
            At::D => "M683.972 465.016v-386.711c-2.636-41.27-36.754-73.744-78.456-73.744s-75.82 32.474-78.445 73.514l-0.012 0.23v764.988c-0 0-0 0-0 0.001 0 43.247 35.059 78.306 78.306 78.306 0.106 0 0.212-0 0.318-0.001l-0.016 0c0.068 0 0.147 0 0.227 0 10.82 0 21.097-2.329 30.355-6.513l-0.465 0.188c-32.753-54.79-52.119-120.857-52.119-191.447 0-99.528 38.499-190.064 101.417-257.529l-0.206 0.223z"
        }],
        path![attrs! {
            At::D => "M817.092 371.351c42.987-18.759 93.047-29.807 145.652-30.117l0.117-0.001h8.433l-60.235-262.325c-8.294-35.054-39.322-60.736-76.348-60.736-43.274 0-78.355 35.081-78.355 78.355 0 6.248 0.731 12.325 2.113 18.151l-0.106-0.532z"
        }],
    ]
}

fn trailer_paths() -> Vec<Node<Msg>> {
    vec![
        path![attrs! {
            At::D => "M813.176 1024h-708.969c-14.3-3.367-24.781-16.017-24.781-31.115 0-0.815 0.031-1.623 0.090-2.422l-0.006 0.107q0-215.642 0-430.984v-4.819c0.015 0 0.033 0 0.051 0 30.976 0 58.991-12.673 79.146-33.116l0.013-0.013c19.218-19.773 31.069-46.796 31.069-76.586 0-1.134-0.017-2.265-0.051-3.391l0.004 0.165h649.939v558.381c-1.037 2.541-2.047 4.621-3.168 6.63l0.157-0.306c-4.8 8.938-13.235 15.394-23.273 17.431l-0.219 0.037zM796.612 481.882h-126.795c-1.944 0.438-3.547 1.646-4.5 3.28l-0.018 0.033-60.235 95.473c-0.466 0.866-0.972 1.957-1.422 3.076l-0.084 0.237h128.301c3.012 0 3.915 0 5.421-3.313l56.922-95.172c0.887-1.056 1.687-2.24 2.356-3.505l0.053-0.11zM393.638 583.078h128.602c0.156 0.017 0.337 0.026 0.52 0.026 2.3 0 4.246-1.517 4.892-3.604l0.010-0.036c18.974-30.118 37.948-62.645 56.621-94.268l2.711-4.518h-125.892c-0.179-0.018-0.387-0.028-0.597-0.028-2.519 0-4.694 1.473-5.711 3.604l-0.016 0.038-58.428 94.268zM377.675 481.882h-126.193c-0.024-0-0.052-0.001-0.080-0.001-2.57 0-4.763 1.609-5.629 3.875l-0.014 0.041-58.428 93.064-2.711 4.216h124.386c0.165 0.018 0.357 0.028 0.551 0.028 2.127 0 3.968-1.225 4.856-3.008l0.014-0.031 60.235-95.473z"
        }],
        path![attrs! {
            At::D => "M707.464 0c4.931 1.519 9.225 3.567 13.143 6.142l-0.192-0.119c4.632 3.831 8.386 8.548 11.033 13.909l0.11 0.247c18.372 44.574 36.442 90.353 54.814 134.325l-602.353 243.652c-18.275-41.26-58.864-69.523-106.054-69.523-14.706 0-28.77 2.745-41.71 7.75l0.79-0.269c-4.819-12.047-10.842-24.094-14.758-37.045-0.883-2.705-1.392-5.818-1.392-9.050 0-13.254 8.561-24.508 20.455-28.534l0.212-0.062c18.673-6.626 39.153-14.456 58.428-20.48l542.118-217.751 43.972-19.275 10.24-3.915zM123.181 271.059h1.807l93.064 67.464c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 90.353-35.84 26.504-10.842-2.409-1.807-91.859-65.656c-0.846-0.572-1.889-0.914-3.012-0.914s-2.166 0.341-3.031 0.926l0.019-0.012-77.402 30.118zM535.793 214.739l-2.711-2.108-90.353-66.56c-0.933-0.622-2.080-0.993-3.313-0.993s-2.38 0.371-3.335 1.007l0.022-0.014-118.061 45.779 2.108 1.807 92.461 67.162c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 87.341-34.635zM730.353 135.529h-1.807l-91.859-68.969c-0.803-0.547-1.794-0.874-2.861-0.874s-2.059 0.327-2.879 0.885l0.018-0.011-90.353 36.744c-8.433 3.012-16.565 6.325-24.998 9.939l2.409 2.108 90.353 65.355c0.846 0.357 1.829 0.565 2.861 0.565s2.015-0.208 2.911-0.583l-0.050 0.018 75.294-30.118z"
        }],
        path![attrs! {
            At::D => "M0 433.393c0-3.614 1.506-7.228 2.409-10.541 8.935-34.682 39.932-59.894 76.818-59.894 4.782 0 9.465 0.424 14.014 1.236l-0.48-0.071c37.902 5.909 66.564 38.317 66.564 77.421 0 2.432-0.111 4.839-0.328 7.214l0.023-0.305c-3.944 40.578-37.878 72.037-79.159 72.037-39.144 0-71.681-28.287-78.286-65.534l-0.070-0.48c-0.474-1.046-0.977-1.935-1.547-2.775l0.041 0.064z"
        }],
    ]
}

fn share_paths() -> Vec<Node<Msg>> {
    vec![
        path![attrs! {
            At::D => "M846.005 679.454c-62.726 0.19-117.909 32.308-150.171 80.95l-0.417 0.669-295.755-96.979c2.298-11.196 3.614-24.064 3.614-37.239 0-0.038-0-0.075-0-0.113l0 0.006c0-0.039 0-0.085 0-0.132 0-29.541-6.893-57.472-19.159-82.272l0.486 1.086 221.967-143.059c42.092 37.259 97.727 60.066 158.685 60.235l0.035 0c0.81 0.010 1.768 0.016 2.726 0.016 128.794 0 233.38-103.646 234.901-232.079l0.001-0.144c0-131.737-106.794-238.532-238.532-238.532s-238.532 106.794-238.532 238.532h0c0.012 33.532 7.447 65.325 20.752 93.828l-0.573-1.367-227.087 146.372c-32.873-23.074-73.687-36.92-117.729-37.045l-0.031-0c-0.905-0.015-1.974-0.023-3.044-0.023-108.186 0-196.124 86.69-198.139 194.395l-0.003 0.189c2.017 107.893 89.956 194.583 198.142 194.583 1.070 0 2.139-0.008 3.205-0.025l-0.161 0.002c0.108 0 0.235 0 0.363 0 60.485 0 114.818-26.336 152.159-68.168l0.175-0.2 313.826 103.002c-0.004 0.448-0.006 0.976-0.006 1.506 0 98.47 79.826 178.296 178.296 178.296s178.296-79.826 178.296-178.296c0-98.468-79.823-178.293-178.29-178.296l-0-0zM923.106 851.727c0.054 1.079 0.084 2.343 0.084 3.614 0 42.748-34.654 77.402-77.402 77.402s-77.402-34.654-77.402-77.402c0-42.748 34.654-77.402 77.402-77.402 0.076 0 0.152 0 0.229 0l-0.012-0c0.455-0.010 0.99-0.015 1.527-0.015 41.12 0 74.572 32.831 75.572 73.711l0.002 0.093zM626.748 230.4c3.537-73.358 63.873-131.495 137.788-131.495s134.251 58.137 137.776 131.179l0.012 0.316c-3.537 73.358-63.873 131.495-137.788 131.495s-134.251-58.137-137.776-131.179l-0.012-0.316zM301.176 626.748c-1.34 53.35-44.907 96.087-98.456 96.087-0.54 0-1.078-0.004-1.616-0.013l0.081 0.001c-1.607 0.096-3.486 0.151-5.377 0.151-53.061 0-96.075-43.014-96.075-96.075s43.014-96.075 96.075-96.075c1.892 0 3.77 0.055 5.635 0.162l-0.258-0.012c0.459-0.008 1-0.012 1.543-0.012 53.443 0 96.943 42.568 98.445 95.648l0.003 0.139z"
        }],
    ]
}

#[view]
fn action_button(
    title: &str, 
    view_box: &str, 
    icon: &str, 
    paths: Vec<Node<Msg>>,
    on_click: impl Into<Option<EventHandler<Msg>>>
) -> Node<Msg> {
    div![
        C!["action-button", "action-button-container", "button-container"],
        s()
            .style_other(":not(:last-child)")
            .margin_right(rem(2)),
        s()
            .flex(CssFlex::None)
            .height(rem(6))
            .margin("2rem 0")
            .width(rem(6))
            .background_color(Color::SurfaceLight5_20)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .justify_content(CssJustifyContent::Center)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::Accent3),
        attrs! {
            At::TabIndex => 0,
            At::Title => title,
        },
        on_click.into().unwrap_or_else(on_click_not_implemented),
        div![
            C!["icon-container",],
            s()
                .align_self(CssAlignSelf::Stretch)
                .flex("0 0 50%")
                .padding_top(pc(15)),
            svg![
                C!["icon",],
                s()
                    .display(CssDisplay::Block)
                    .fill(Color::SurfaceLight5_90)
                    .height(pc(100))
                    .width(pc(100))
                    .overflow(CssOverflow::Visible),
                attrs! {
                    At::ViewBox => view_box,
                    "icon" => icon,
                },
                paths,
            ],
        ],
        div![
            C!["label-container",], 
            s()
                .align_items(CssAlignItems::Center)
                .align_self(CssAlignSelf::Stretch)
                .display(CssDisplay::Flex)
                .flex("0 0 50%")
                .flex_direction(CssFlexDirection::Row),
            div![
                C!["label"], 
                s()
                    .color(Color::SurfaceLight5_90)
                    .flex("1")
                    .font_weight("500")
                    .max_height(em(2.4))
                    .padding("0 0.2rem")
                    .text_align(CssTextAlign::Center),
                title,
            ]
        ]
    ]
}
