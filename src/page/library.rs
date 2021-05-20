use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::rc::Rc;
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Events, Urls as RootUrls, ActionCtx};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg};
use stremio_core::types::library::LibraryItem;
use stremio_core::types::resource::PosterShape;
use stremio_core::models::library_with_filters::{
    Selected as LibraryWithFiltersSelected, 
    LibraryWithFilters, 
    LibraryRequest, 
    LibraryRequestPage,
    Sort,
};

mod type_selector;
mod sort_selector;

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    context: &mut Context,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    let base_url = url.to_hash_base_url();

    let library_request = match url.remaining_hash_path_parts().as_slice() {
        [type_, sort, page] => {
            (|| Some(LibraryRequest {
                r#type: if *type_ == "all" { None } else { Some(type_.to_string()) },
                sort: match *sort {
                    "last_watched" => Sort::LastWatched,
                    "name" => Sort::Name,
                    "times_watched" => Sort::TimesWatched,
                    _ => None?
                },
                page: LibraryRequestPage(page.parse().ok()?),
            }))()
        }
        _ => None,
    };

    load_library(library_request.clone(), orders);

    let mut model = model.get_or_insert_with(move || Model {
        base_url,
        library_request: None,
        selected_library_item: None,
        _events_sub_handle: orders.subscribe_with_handle(|events| {
            Some(match events {
                Events::CtxLoaded => Msg::ReloadLibrary,
                Events::WindowClicked => Msg::WindowClicked,
                _ => return None
            })
        }),
    });
    model.library_request = library_request;
    Some(PageId::Library)
}

fn load_library(library_request: Option<LibraryRequest>, orders: &mut impl Orders<Msg>) {
    let selected_library = LibraryWithFiltersSelected {
        request: library_request.unwrap_or_else(default_library_request)
    };
    orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::LibraryWithFilters(selected_library),
    )))));
}

pub fn default_library_request() -> LibraryRequest {
    LibraryRequest {
        r#type: None,
        sort: Sort::default(),
        page: LibraryRequestPage::default(),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    library_request: Option<LibraryRequest>,
    selected_library_item: Option<String>,
    _events_sub_handle: SubHandle,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
    pub fn library_request(self, library_request: &LibraryRequest) -> Url {
        let type_ = library_request
            .r#type
            .as_ref()
            .map(String::as_str)
            .unwrap_or("all");

        let sort = match library_request.sort {
            Sort::LastWatched => "last_watched",
            Sort::Name => "name",
            Sort::TimesWatched => "times_watched",
        };

        let page = library_request.page.to_string();

        self.base_url()
            .add_hash_path_part(type_)
            .add_hash_path_part(sort)
            .add_hash_path_part(page)
    }
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ReloadLibrary,
    SelectLibraryItem(Option<String>),
    GoToDetails(Url),
    RemoveLibraryItem(String),
    WindowClicked,
    SendLibraryRequest(LibraryRequest),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ReloadLibrary => {
            load_library(model.library_request.clone(), orders)
        }
        Msg::SelectLibraryItem(id) => {
            model.selected_library_item = id;
        }
        Msg::GoToDetails(details_url) => {
            orders.request_url(details_url);
        }
        Msg::RemoveLibraryItem(library_item_id) => {
            orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Ctx(
                ActionCtx::RemoveFromLibrary(library_item_id)
            )))));
        }
        Msg::WindowClicked => {
            model.selected_library_item = None;
        }
        Msg::SendLibraryRequest(library_request) => {
            orders.request_url(Urls::new(&model.base_url).library_request(&library_request));
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Vec<Node<RootMsg>> {
    basic_layout(BasicLayoutArgs {
        page_content: library_content(model, context).map_msg(msg_mapper),
        container_class: "library-container",
        context,
        page_id,
        search_args: None,
        modal: None,
    })
}

#[view]
fn library_content<'a>(model: &Model, context: &Context) -> Node<Msg> {
    let library_items = &context.core_model.library.catalog;
    div![
        C!["library-content"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .height(pc(100))
            .width(pc(100)),  
        if context.core_model.ctx.profile.auth.is_some() {
            nodes![
                selectable_inputs(model, context),
                if library_items.is_empty() {
                    message_container(MessageContainer::EmptyLibrary)
                } else {
                    meta_items_container(
                        library_items, 
                        model.selected_library_item.as_ref(), 
                        &context.root_base_url
                    )
                }
            ]
        } else {
            vec![
                message_container(MessageContainer::Login(&context.root_base_url))
            ]
        }          
    ]
}

#[view]
fn selectable_inputs(model: &Model, context: &Context) -> Node<Msg> {
    let library = &context.core_model.library;
    div![
        C!["selectable-inputs-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .overflow(CssOverflow::Visible)
            .padding(rem(1.5)),
        type_selector::view(library, Msg::SendLibraryRequest),
        sort_selector::view(library, Msg::SendLibraryRequest),
        div![
            C!["spacing"],
            s()
                .flex("1"),
        ],
        pagination_input(library),
    ]
}

#[view]
fn pagination_input<F>(library: &LibraryWithFilters<F>) -> Node<Msg> {
    div![
        C!["pagination-input", "pagination-input-container"],
        s()
            .flex(CssFlex::None)
            .height(rem(3.5))
            .margin_left(rem(1.5))
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row),
        pagination_prev_button(library.selectable.prev_page.as_ref().map(|page| &page.request)),
        pagination_label(library.selected.as_ref()),
        pagination_next_button(library.selectable.next_page.as_ref().map(|page| &page.request)),
    ]
}

#[view]
fn pagination_label(library_selected: Option<&LibraryWithFiltersSelected>) -> Node<Msg> {
    let page_number = library_selected.map(|selected| {
        selected.request.page.0.get() 
    }).unwrap_or(1);
    div![
        C!["label-container"],
        s()
            .align_items(CssAlignItems::Center)
            .align_self(CssAlignSelf::Stretch)
            .background_color(Color::BackgroundDark1)
            .display(CssDisplay::Flex)
            .flex("1")
            .justify_content(CssJustifyContent::Center),
        attrs!{
            At::Title => page_number,
        },
        div![
            C!["label"],
            s()
                .width(rem(3))
                .color(Color::SecondaryVariant1_90)
                .flex(CssFlex::None)
                .font_weight("500")
                .max_width(rem(3))
                .min_width(rem(1.2))
                .text_align(CssTextAlign::Center)
                .text_overflow("ellipsis")
                .white_space(CssWhiteSpace::NoWrap),
            page_number,
        ]
    ]
}

#[view]
fn pagination_prev_button(previous_page_request: Option<&LibraryRequest>) -> Node<Msg> {
    let disabled = previous_page_request.is_none();
    div![
        C!["prev-button-container", "button-container"],
        attrs!{
            At::TabIndex => 0,
            At::Title => "Previous page",
        },
        s()
            .height(rem(3.5))
            .width(rem(3.5))
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .justify_content(CssJustifyContent::Center)
            .cursor(CssCursor::Pointer),
        IF!(disabled => s().pointer_events("none")),
        previous_page_request.cloned().map(|request| {
            ev(Ev::Click, move |_| Msg::SendLibraryRequest(request.clone()))
        }),
        svg![
            C!["icon"],
            s()
                .height(rem(1))
                .width(rem(1))
                .display(CssDisplay::Block)
                .fill(Color::SecondaryVariant1_90)
                .overflow(CssOverflow::Visible),
            IF!(disabled => s().fill(Color::SurfaceDark5_90)),
            attrs!{
                At::ViewBox => "0 0 606 1024",
                At::from("icon") => "ic_arrow_left",
            },
            path![
                attrs!{
                    At::D => "M264.132 512l309.609-319.247c19.848-20.685 32.069-48.821 32.069-79.812s-12.221-59.127-32.107-79.852l0.038 0.040c-19.51-20.447-46.972-33.16-77.402-33.16s-57.892 12.713-77.363 33.118l-0.040 0.042-387.012 399.059c-19.713 20.744-31.839 48.862-31.839 79.812s12.126 59.067 31.886 79.861l-0.047-0.050 387.012 399.059c19.51 20.447 46.972 33.16 77.402 33.16s57.892-12.713 77.363-33.118l0.040-0.042c19.848-20.685 32.069-48.821 32.069-79.812s-12.221-59.127-32.107-79.852l0.038 0.040z",
                }
            ]
        ]
    ]
}

#[view]
fn pagination_next_button(next_page_request: Option<&LibraryRequest>) -> Node<Msg> {
    let disabled = next_page_request.is_none();
    div![
        C!["next-button-container", "button-container"],
        attrs!{
            At::TabIndex => 0,
            At::Title => "Next page",
        },
        s()
            .height(rem(3.5))
            .width(rem(3.5))
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .justify_content(CssJustifyContent::Center)
            .cursor(CssCursor::Pointer),
        IF!(disabled => s().pointer_events("none")),
        next_page_request.cloned().map(|request| {
            ev(Ev::Click, move |_| Msg::SendLibraryRequest(request.clone()))
        }),
        svg![
            C!["icon"],
            s()
                .height(rem(1))
                .width(rem(1))
                .display(CssDisplay::Block)
                .fill(Color::SecondaryVariant1_90)
                .overflow(CssOverflow::Visible),
            IF!(disabled => s().fill(Color::SurfaceDark5_90)),
            attrs!{
                At::ViewBox => "0 0 606 1024",
                At::from("icon") => "ic_arrow_left",
            },
            path![
                attrs!{
                    At::D => "M341.534 512l-309.609-319.247c-19.713-20.744-31.839-48.862-31.839-79.812s12.126-59.067 31.886-79.861l-0.047 0.050c19.51-20.447 46.972-33.16 77.402-33.16s57.892 12.713 77.363 33.118l0.040 0.042 387.012 399.059c19.848 20.685 32.069 48.821 32.069 79.812s-12.221 59.127-32.107 79.852l0.038-0.040-387.012 399.059c-19.51 20.447-46.972 33.16-77.402 33.16s-57.892-12.713-77.363-33.118l-0.040-0.042c-19.713-20.744-31.839-48.862-31.839-79.812s12.126-59.067 31.886-79.861l-0.047 0.050z",
                }
            ]
        ]
    ]
}

#[view]
fn meta_items_container(library_items: &[LibraryItem], selected_library_item: Option<&String>, root_base_url: &Url) -> Node<Msg> {
    div![
        C!["meta-items-container"],
        s()
            .align_items(CssAlignItems::Center)
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Grid)
            .flex("1")
            .grid_auto_rows("max-content")
            .grid_gap(rem(0.5))
            .margin_right(rem(1.5))
            .overflow_y(CssOverflowY::Auto)
            .padding("0 1.5rem"),
        s()
            .only_and_below(Breakpoint::Normal)
            .grid_template_columns("repeat(9, 1fr)"),
        s()
            .only_and_below(Breakpoint::Medium)
            .grid_template_columns("repeat(8, 1fr)"),
        s()
            .only_and_below(Breakpoint::Small)
            .grid_template_columns("repeat(7, 1fr)"),
        s()
            .only_and_below(Breakpoint::XSmall)
            .grid_template_columns("repeat(6, 1fr)"),
        s()
            .only_and_below(Breakpoint::XXSmall)
            .grid_template_columns("repeat(5, 1fr)"),
        s()
            .only_and_below(Breakpoint::Minimum)
            .grid_template_columns("repeat(4, 1fr)"),
        library_items
            .iter()
            .map(|item| library_item(item, selected_library_item, root_base_url)),
    ]
}

#[view]
fn library_item(item: &LibraryItem, selected_library_item: Option<&String>, root_base_url: &Url) -> Node<Msg> {
    let square = matches!(item.poster_shape, PosterShape::Square);
    let selected = selected_library_item == Some(&item.id);
    let details_url = RootUrls::new(root_base_url).detail_urls().without_video_id(&item.r#type, &item.id);
    a![
        el_key(&item.id),
        C!["meta-item", "poster-shape-poster", "meta-item-container", "button-container", IF!(square => "poster-shape-square")],
        s()
            .flex(format!("calc(1 / {});", global::POSTER_SHAPE_RATIO).as_str())
            .padding(rem(1))
            .overflow(CssOverflow::Visible)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight3)
            .transition("background-color 100ms ease-out"),
        IF!(selected => {
            s()
                .background_color(Color::BackgroundLight3)
                .transition("background-color 100ms ease-out")
        }),
        attrs!{
            At::TabIndex => 0,
            At::Title => item.name,
            At::Href => &details_url,
        },
        poster_container(&item.poster, square, selected),
        div![
            C!["title-bar-container"],
            s()
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .height(rem(2.8))
                .overflow(CssOverflow::Visible),
            div![
                C!["title-label"],
                s()
                    .padding_right(rem(0.5))
                    .color(Color::SurfaceLight5_90)
                    .flex("1")
                    .max_height(em(2.4))
                    .padding_left(rem(0.5)),
                &item.name,
            ],
            menu_label_container(selected, &item.id, details_url),
        ]
    ]
}

#[view]
fn menu_label_container(selected: bool, library_item_id: &str, details_url: Url) -> Node<Msg> {
    div![
        C!["menu-label-container", "label-container", "button-container", IF!(selected => "active")],
        s()
            .background_color("transparent")
            .flex(CssFlex::None)
            .height(rem(2.8))
            .padding("1rem 0")
            .width(rem(1.5))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative)
            .cursor(CssCursor::Pointer),
        attrs!{
            At::TabIndex => -1,
        },
        {
            let library_item_id = library_item_id.to_owned();
            ev(Ev::Click, move |event| {
                event.prevent_default();
                event.stop_propagation();
                Msg::SelectLibraryItem(if selected { None } else { Some(library_item_id) })
            })
        },
        svg![
            C!["icon"],
            s()
                .display(CssDisplay::Block)
                .fill(Color::SurfaceLight1_90)
                .height(pc(100))
                .width(pc(100))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => "0 0 216 1024",
                At::from("icon") => "ic_more",
            },
            path![
                attrs!{
                    At::D => "M215.944 108.122c0-0.089 0-0.195 0-0.301 0-59.714-48.408-108.122-108.122-108.122s-108.122 48.408-108.122 108.122c0 59.714 48.408 108.122 108.122 108.122 0.106 0 0.211-0 0.317-0l-0.016 0c59.548 0 107.821-48.273 107.821-107.821v0z",
                }
            ],
            path![
                attrs!{
                    At::D => "M215.944 507.181c-0-59.714-48.408-108.122-108.122-108.122s-108.122 48.408-108.122 108.122c0 59.714 48.408 108.122 108.122 108.122 0.106 0 0.212-0 0.318-0l-0.016 0c0 0 0 0 0 0 59.548 0 107.821-48.273 107.821-107.821 0-0.106-0-0.212-0-0.318l0 0.017z",
                }
            ],
            path![
                attrs!{
                    At::D => "M215.944 915.878c-0-59.714-48.408-108.122-108.122-108.122s-108.122 48.408-108.122 108.122c0 59.714 48.408 108.122 108.122 108.122 0.106 0 0.212-0 0.318-0l-0.016 0c0 0 0 0 0 0 59.548 0 107.821-48.273 107.821-107.821 0-0.106-0-0.212-0-0.318l0 0.017z",
                }
            ],
        ],
        IF!(selected => {
            menu_container(details_url, library_item_id)
        })
    ]
}

#[view]
fn poster_container(poster: &Option<String>, square: bool, selected: bool) -> Node<Msg> {
    let padding_top = if square { 
        pc(100).to_string() 
    } else { 
        format!("calc(100% * {})", global::POSTER_SHAPE_RATIO)
    };
    div![
        C!["poster-container"],
        s()
            .padding_top(padding_top.as_str())
            .background_color(Color::Background)
            .position(CssPosition::Relative)
            .z_index("0"),
        div![
            C!["poster-image-layer"],
            s()
                .align_items(CssAlignItems::Center)
                .bottom("0")
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .justify_content(CssJustifyContent::Center)
                .left("0")
                .position(CssPosition::Absolute)
                .right("0")
                .top("0")
                .z_index("-3"),
            img![
                C!["poster-image"],
                s()
                    .flex(CssFlex::None)
                    .height(pc(100))
                    .object_fit("cover")
                    .object_position("center")
                    .opacity("0.9")
                    .width(pc(100)),
                attrs!{
                    At::Alt => " ",
                    At::Src => poster.clone().unwrap_or_default(),
                },
            ]
        ],
    ]
}

#[view]
fn menu_container(details_url: Url, library_item_id: &str) -> Node<Msg> {
    let library_item_id = library_item_id.to_owned();
    div![
        C!["menu-container", "menu-direction-bottom-right"],
        s()
            .width(CssWidth::Auto)
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
            C!["menu-container"],
            s()
                .max_width(rem(12))
                .min_width(rem(6)),
            option_container("Details", ev(Ev::Click, move |_| Msg::GoToDetails(details_url))), 
            option_container("Dismiss",  None), 
            option_container("Remove", ev(Ev::Click, move |_| Msg::RemoveLibraryItem(library_item_id))), 
        ]
    ]
}

#[view]
fn option_container(title: &str, on_click: impl Into<Option<EventHandler<Msg>>>) -> Node<Msg> {
    div![
        C!["option-container", "button-container"],
        s()
            .background_color(Color::SurfaceLight5)
            .padding(rem(0.5))
            .align_items(CssAlignItems::Center)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::SurfaceLight2),
        attrs!{
            At::TabIndex => 0,
            At::Title => title, 
        },
        on_click.into(),
        div![
            C!["label"],
            s()
                .color(hsla(0, 0, 0, 0.9))
                .flex("1")
                .max_height(rem(4.8)),
            title,
        ]
    ]
}


enum MessageContainer<'a> {
    Login(&'a Url),
    EmptyLibrary,
}

#[view]
fn message_container(message_container: MessageContainer) -> Node<Msg> {
    div![
        C!["message-container", "no-user-message-container"],
        s()
            .padding(rem(4))
            .align_items(CssAlignItems::Center)
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex("0 1 auto")
            .flex_direction(CssFlexDirection::Column)
            .overflow_y(CssOverflowY::Auto),
        img![
            C!["image"],
            s()
                .flex(CssFlex::None)
                .height(rem(12))
                .margin_bottom(rem(1))
                .raw(r#"object-fit: contain;"#)
                .raw(r#"object-position: center;"#)
                .opacity("0.9")
                .width(rem(12)),
            attrs!{
                At::Src => if let MessageContainer::Login(root_url_base) = message_container {
                    global::image_url("anonymous.png")
                } else {
                    global::image_url("empty.png")
                }
                At::Alt => "",
            },
        ],
        if let MessageContainer::Login(root_url_base) = message_container {
            login_button(root_url_base)
        } else {
            empty![]
        },
        div![
            C!["message-label"],
            s()
                .color(Color::SecondaryVariant2Light1_90)
                .flex(CssFlex::None)
                .font_size(rem(2.5))
                .text_align(CssTextAlign::Center),
            if let MessageContainer::Login(root_url_base) = message_container {
                "Library is only available for logged in users!"
            } else {
                "Empty Library"
            },
        ]
    ]
}

#[view]
fn login_button(root_url_base: &Url) -> Node<Msg> {
    a![
        C!["login-button-container", "button-container"],
        s()
            .align_items(CssAlignItems::Center)
            .background_color(Color::Accent3)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .justify_content(CssJustifyContent::Center)
            .margin_bottom(rem(1))
            .min_height(rem(4))
            .padding("0.5rem 1rem")
            .width(rem(20))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .color(Color::Accent3Light1),
        attrs!{
            At::Href => RootUrls::new(root_url_base).intro(),
            At::TabIndex => "0",
        },
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex_basis(CssFlexBasis::Auto)
                .flex_grow("0")
                .flex_shrink("1")
                .font_size(rem(1.2))
                .font_weight("700")
                .max_height(em(4.8))
                .text_align(CssTextAlign::Center),
            "LOG IN",
        ]
    ]
}
