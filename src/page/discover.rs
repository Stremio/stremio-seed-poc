use crate::{entity::multi_select, Context, PageId, Actions, Urls as RootUrls};
use enclose::enc;
use seed::{prelude::*, *};
use std::rc::Rc;
use stremio_core::runtime::msg::{Msg as CoreMsg, Action, Internal, Event, ActionLoad};
use stremio_core::models::common::{Loadable, ResourceError};
use stremio_core::models::catalog_with_filters::Selected as CatalogWithFiltersSelected;
use stremio_core::types::resource::{MetaItemPreview, PosterShape};
use stremio_core::types::addon::{ResourceRequest, ResourceResponse, ResourcePath};
use seed_styles::{px, pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};

mod catalog_selector;
mod extra_prop_selector;
mod type_selector;

type MetaItemPreviewId = String;
// @TODO add into stremio-core?
type ExtraPropOption = String;

const DEFAULT_RESOURCE: &str = "catalog";
const DEFAULT_TYPE: &str = "movie";
const DEFAULT_ID: &str = "top";
const BASE: &str = "https://v4-cinemeta.strem.io/manifest.json";

fn on_click_not_implemented() -> EventHandler<Msg> {
    ev(Ev::Click, |_| { window().alert_with_message("Not implemented!"); })
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

    let resource_request = match url.remaining_hash_path_parts().as_slice() {
        [base, path] => {
            (|| Some(ResourceRequest::new(
                base.parse().map_err(|error| error!(error)).ok()?,
                serde_json::from_str(path).map_err(|error| error!(error)).ok()?
            )))()
        }
        _ => None,
    };

    load_catalog(resource_request, orders);

    model.get_or_insert_with(move || Model {
        base_url,
        _core_msg_sub_handle: orders.subscribe_with_handle(Msg::CoreMsg),
        selected_meta_preview_id: None,
    });
    Some(PageId::Discover)
}

fn load_catalog(resource_request: Option<ResourceRequest>, orders: &mut impl Orders<Msg>) {
    let selected_catalog = CatalogWithFiltersSelected {
        request: resource_request.unwrap_or_else(default_resource_request)
    };
    orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::CatalogWithFilters(selected_catalog),
    )))));
}

pub fn default_resource_request() -> ResourceRequest {
    ResourceRequest::new(
        BASE.parse().expect("valid BASE url"),
        ResourcePath::without_extra(DEFAULT_RESOURCE, DEFAULT_TYPE, DEFAULT_ID),
    )
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    _core_msg_sub_handle: SubHandle,
    selected_meta_preview_id: Option<MetaItemPreviewId>,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn root(self) -> Url {
        self.base_url()
    }
    pub fn res_req(self, res_req: &ResourceRequest) -> Url {
        self.base_url()
            .add_hash_path_part(res_req.base.to_string())
            .add_hash_path_part(serde_json::to_string(&res_req.path).unwrap())
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::pub_enum_variant_names, clippy::large_enum_variant)]
pub enum Msg {
    CoreMsg(Rc<CoreMsg>),
    MetaItemPreviewClicked(MetaItemPreview),
    // TypeSelectorMsg(type_selector::Msg),
    // TypeSelectorChanged(Vec<multi_select::Item>),
    // CatalogSelectorMsg(catalog_selector::Msg),
    // CatalogSelectorChanged(Vec<multi_select::Item>),
    // ExtraPropSelectorMsg(extra_prop_selector::Msg),
    // ExtraPropSelectorChanged(Vec<multi_select::Item>),
    SendResourceRequest(ResourceRequest)
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    let catalog = &context.core_model.catalog;

    match msg {
        Msg::CoreMsg(core_msg) => {
            if let CoreMsg::Internal(Internal::ResourceRequestResult(_, result)) = core_msg.as_ref() {
                if let Ok(ResourceResponse::Metas { metas }) = result.as_ref() {
                    model.selected_meta_preview_id = metas.first().map(|meta| meta.id.clone());
                }
            }
        }
        Msg::MetaItemPreviewClicked(meta_preview) => {
            // if model.selected_meta_preview_id.as_ref() == Some(&meta_preview.id) {
            //     let id = &meta_preview.id;
            //     let type_name = &meta_preview.type_name;

            //     let detail_urls = RootUrls::new(&context.root_base_url).detail_urls();

            //     orders.request_url(if meta_preview.type_name == "movie" {
            //         detail_urls.with_video_id(type_name, id, id)
            //     } else {
            //         detail_urls.without_video_id(type_name, id)
            //     });
            // } else {
            //     model.selected_meta_preview_id = Some(meta_preview.id);
            // }
        }

        Msg::SendResourceRequest(res_req) => {
            orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        }

        // ------ TypeSelector  ------
        // Msg::TypeSelectorMsg(msg) => {
        //     // let msg_to_parent = type_selector::update(
        //     //     msg,
        //     //     &mut model.type_selector_model,
        //     //     &mut orders.proxy(Msg::TypeSelectorMsg),
        //     //     type_selector::groups(&catalog.types),
        //     //     Msg::TypeSelectorChanged,
        //     // );
        //     // if let Some(msg) = msg_to_parent {
        //     //     orders.send_msg(msg);
        //     // }
        // }
        // Msg::TypeSelectorChanged(groups_with_selected_items) => {
        //     // let res_req = type_selector::resource_request(groups_with_selected_items);
        //     // orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        // }

        // // ------ CatalogSelector  ------
        // Msg::CatalogSelectorMsg(msg) => {
        //     // let msg_to_parent = catalog_selector::update(
        //     //     msg,
        //     //     &mut model.catalog_selector_model,
        //     //     &mut orders.proxy(Msg::CatalogSelectorMsg),
        //     //     catalog_selector::items(&catalog.catalogs, &catalog.selected),
        //     //     Msg::CatalogSelectorChanged,
        //     // );
        //     // if let Some(msg) = msg_to_parent {
        //     //     orders.send_msg(msg);
        //     // }
        // }
        // Msg::CatalogSelectorChanged(groups_with_selected_items) => {
        //     // let res_req = catalog_selector::resource_request(groups_with_selected_items);
        //     // orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        // }

        // // ------ ExtraPropSelector  ------
        // Msg::ExtraPropSelectorMsg(msg) => {
        //     // let msg_to_parent = extra_prop_selector::update(
        //     //     msg,
        //     //     &mut model.extra_prop_selector_model,
        //     //     &mut orders.proxy(Msg::ExtraPropSelectorMsg),
        //     //     extra_prop_selector::groups(&catalog.selectable_extra, &catalog.selected),
        //     //     Msg::ExtraPropSelectorChanged,
        //     // );
        //     // if let Some(msg) = msg_to_parent {
        //     //     orders.send_msg(msg);
        //     // }
        // }
        // Msg::ExtraPropSelectorChanged(groups_with_selected_items) => {
        //     // if let Some(res_req) =
        //     //     extra_prop_selector::resource_request(groups_with_selected_items, &catalog.selected)
        //     // {
        //     //     orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        //     // }
        // }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context) -> Node<Msg> {
    div![
        C!["discover-container", "main-nav-bars-container"],
        s()
            .height(pc(100))
            .width(pc(100))
            .background_color(Color::BackgroundDark2)
            .position(CssPosition::Relative)
            .z_index("0"),
        horizontal_nav_bar(),
        vertical_nav_bar(),
        div![
            C!["nav-content-container"],
            s()
                .position(CssPosition::Absolute)
                .bottom("0")
                .left(global::VERTICAL_NAV_BAR_SIZE)
                .right("0")
                .top(global::HORIZONTAL_NAV_BAR_SIZE)
                .z_index("0"),
            div![
                C!["discover-content"],
                s()
                    .display(CssDisplay::Flex)
                    .flex_direction(CssFlexDirection::Row)
                    .height(pc(100))
                    .width(pc(100)),            
                div![
                    C!["catalog-container"],
                    s()
                        .align_self(CssAlignSelf::Stretch)
                        .display(CssDisplay::Flex)
                        .flex("1")
                        .flex_direction(CssFlexDirection::Column),
                    selectable_inputs(model, context),
                    context.core_model.catalog.catalog.as_ref().map(|resource_loadable| {
                        meta_items(
                            &resource_loadable.content,
                            model.selected_meta_preview_id.as_ref()
                        )
                    }),
                ],
                div![
                    C!["meta-preview-container", "compact"],
                    s()
                        .align_self(CssAlignSelf::Stretch)
                        .background_color(Color::BackgroundDark3)
                        .flex(CssFlex::None)
                        .width(rem(28))
                        .display(CssDisplay::Flex)
                        .flex_direction(CssFlexDirection::Column)
                        .position(CssPosition::Relative)
                        .z_index("0"),
                    s()
                        .only_and_below(Breakpoint::XXSmall)
                        .display(CssDisplay::None),
                ]
            ],
        ]
    ]
}

#[view]
fn horizontal_nav_bar() -> Node<Msg> {
    nav![
        C!["horizontal-nav-bar", "horizontal-nav-bar-container"],
        s()
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("0")
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .height(global::HORIZONTAL_NAV_BAR_SIZE)
            .overflow(CssOverflow::Visible)
            .padding_right(rem(1)),
        // .. @TODO implemented in the Search page
    ]
}

#[view]
fn vertical_nav_bar() -> Node<Msg> {
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
        // .. @TODO implemented in the Search page
    ]
}

#[view]
fn selectable_inputs(model: &Model, context: &Context) -> Node<Msg> {
    let catalog = &context.core_model.catalog;

    div![
        C!["selectable-inputs-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .overflow(CssOverflow::Visible)
            .padding(rem(1.5)),
        type_selector::view(catalog, Msg::SendResourceRequest),
        catalog_selector::view(catalog, Msg::SendResourceRequest),
        extra_prop_selector::view(catalog, Msg::SendResourceRequest),
        div![
            C!["spacing"],
            s()
                .flex("1"),
        ],
        pagination_input(),
    ]
}

#[view]
fn pagination_input() -> Node<Msg> {
    div![
        C!["pagination-input", "pagination-input-container"],
        s()
            .flex(CssFlex::None)
            .height(rem(3.5))
            .margin_left(rem(1.5))
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row),
        pagination_prev_button(),
        pagination_label(),
        pagination_next_button(),
    ]
}

#[view]
fn pagination_label() -> Node<Msg> {
    let page_number = 1;
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
fn pagination_prev_button() -> Node<Msg> {
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
        svg![
            C!["icon"],
            s()
                .height(rem(1))
                .width(rem(1))
                .display(CssDisplay::Block)
                .fill(Color::SecondaryVariant1_90)
                .overflow(CssOverflow::Visible),
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
fn pagination_next_button() -> Node<Msg> {
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
        svg![
            C!["icon"],
            s()
                .height(rem(1))
                .width(rem(1))
                .display(CssDisplay::Block)
                .fill(Color::SecondaryVariant1_90)
                .overflow(CssOverflow::Visible),
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
fn meta_items(
    content: &Loadable<Vec<MetaItemPreview>, ResourceError>,
    selected_meta_preview_id: Option<&MetaItemPreviewId>,
) -> Node<Msg> {
    let message_container_style = s()
        .padding("0 2rem")
        .font_size(rem(2))
        .color(Color::SurfaceLighter);

    match content {
        Loadable::Err(resource_error) => {
            div![C!["message-container",], message_container_style, format!("{}", resource_error)]
        }
        Loadable::Loading => div![C!["message-container",], message_container_style, "Loading"],
        Loadable::Ready(meta_previews) if meta_previews.is_empty() => empty![],
        Loadable::Ready(meta_previews) => div![
            C!["meta-items-container",],
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
                .only_and_above(Breakpoint::XLarge)
                .grid_template_columns("repeat(10, 1fr)"),
            s()
                .only_and_below(Breakpoint::Large)
                .grid_template_columns("repeat(8, 1fr)"),
            s()
                .only_and_below(Breakpoint::Normal)
                .grid_template_columns("repeat(7, 1fr)"),
            s()
                .only_and_below(Breakpoint::Medium)
                .grid_template_columns("repeat(6, 1fr)"),
            s()
                .only_and_below(Breakpoint::Small)
                .grid_template_columns("repeat(5, 1fr)"),
            s()
                .only_and_below(Breakpoint::XSmall)
                .grid_template_columns("repeat(4, 1fr)"),
            s()
                .only_and_below(Breakpoint::XXSmall)
                .grid_template_columns("repeat(5, 1fr)"),
            s()
                .only_and_below(Breakpoint::Minimum)
                .grid_template_columns("repeat(4, 1fr)"),
            meta_previews
                .iter()
                .map(|meta_preview| meta_item(meta_preview, selected_meta_preview_id)),
        ],
    }
}

#[view]
fn meta_item(meta: &MetaItemPreview, selected_meta_preview_id: Option<&MetaItemPreviewId>) -> Node<Msg> {
    let square = matches!(meta.poster_shape, PosterShape::Square);
    a![
        el_key(&meta.id),
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
        attrs!{
            At::TabIndex => 0,
            At::Title => meta.name,
        },
        on_click_not_implemented(),
        poster_container(&meta.poster, square),
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
                &meta.name,
            ]
        ]
    ]
}

#[view]
fn poster_container(poster: &Option<String>, square: bool) -> Node<Msg> {
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
        ]
    ]
}
