use seed::{prelude::*, *};
use seed_hooks::{*, topo::nested as view};
use seed_styles::{em, pc, rem, Style};
use seed_styles::*;
use std::rc::Rc;
use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Events, Urls as RootUrls};
use crate::basic_layout::{basic_layout, BasicLayoutArgs};
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use stremio_core::runtime::msg::{Action, ActionLoad, Msg as CoreMsg};
use stremio_core::types::library::LibraryItem;
use stremio_core::models::library_with_filters::{
    Selected as LibraryWithFiltersSelected, 
    LibraryWithFilters, 
    LibraryRequest, 
    LibraryRequestPage,
    Sort,
};

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
        _events_sub_handle: orders.subscribe_with_handle(|Events::LibraryLoadedFromStorage| Msg::ReloadLibrary),
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
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    ReloadLibrary,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ReloadLibrary => {
            load_library(model.library_request.clone(), orders)
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    basic_layout(BasicLayoutArgs {
        page_content: library_content(model, context).map_msg(msg_mapper),
        container_class: "library-container",
        context,
        page_id,
        search_args: None,
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
                selectable_inputs_container(),
                if library_items.is_empty() {
                    message_container(MessageContainer::EmptyLibrary)
                } else {
                    meta_items_container(library_items)
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
fn selectable_inputs_container() -> Node<Msg> {
    div![
        "selectable inputs container",
    ]
}

#[view]
fn meta_items_container(library_items: &[LibraryItem]) -> Node<Msg> {
    div![
        C!["meta-items-container"],
        "LIBRARY ITEMS"
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
