use crate::{multi_select, Msg as RootMsg, Context, PageId, Actions, Urls as RootUrls};
use enclose::enc;
use seed::{prelude::*, *};
use std::rc::Rc;
use stremio_core::runtime::msg::{Msg as CoreMsg, Action, Internal, Event, ActionLoad};
use stremio_core::models::common::{Loadable, ResourceError};
use stremio_core::models::catalog_with_filters::{Selected as CatalogWithFiltersSelected, CatalogWithFilters};
use stremio_core::types::resource::{MetaItemPreview, PosterShape};
use stremio_core::types::addon::{ResourceRequest, ResourceResponse, ResourcePath};
use stremio_core::constants::{CATALOG_PAGE_SIZE, SKIP_EXTRA_NAME};
use seed_styles::{px, pc, rem, em};
use seed_styles::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};
use seed_hooks::{*, topo::nested as view};
use crate::page;

mod catalog_selector;
mod extra_prop_selector;
mod type_selector;

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
        selected_meta_preview: None,
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
    selected_meta_preview: Option<MetaItemPreview>,
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
    SelectMetaItemPreview(MetaItemPreview),
    SendResourceRequest(ResourceRequest)
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CoreMsg(core_msg) => {
            if let CoreMsg::Internal(Internal::ResourceRequestResult(_, result)) = core_msg.as_ref() {
                if let Ok(ResourceResponse::Metas { metas }) = result.as_ref() {
                    // @TODO store only id or index?
                    model.selected_meta_preview = metas.first().cloned();
                }
            }
        }
        Msg::SelectMetaItemPreview(meta_preview) => {
            model.selected_meta_preview = Some(meta_preview);
        }
        Msg::SendResourceRequest(res_req) => {
            orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        }
    }
}

// ------ ------
//     View
// ------ ------

#[view]
pub fn view(model: &Model, context: &Context, page_id: PageId, msg_mapper: fn(Msg) -> RootMsg) -> Node<RootMsg> {
    page::basic_layout(page::BasicLayoutArgs {
        page_content: discover_content(model, context).map_msg(msg_mapper),
        container_class: "discover-container",
        context,
        page_id,
        search_args: None,
    })
}

#[view]
fn discover_content(model: &Model, context: &Context) -> Node<Msg> {
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
                    model.selected_meta_preview.as_ref(),
                    &context.root_base_url,
                )
            }),
        ],
        meta_preview_container(model.selected_meta_preview.as_ref()),
    ]
}

#[view]
fn meta_preview_container(selected_meta_preview: Option<&MetaItemPreview>) -> Node<Msg> {
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
        selected_meta_preview.map(|selected_meta_preview| {
            vec![
                meta_preview_background(selected_meta_preview.poster.as_ref()),
                meta_preview_info(selected_meta_preview),
                meta_preview_buttons(selected_meta_preview),
            ]
        })
    ]
}

#[view]
fn meta_preview_background(image_url: Option<&String>) -> Node<Msg> {
    div![
        C!["background-image-layer"],
        s()
            .bottom(px(-10))
            .left(px(-10))
            .position(CssPosition::Absolute)
            .right(px(-10))
            .top(px(-10))
            .z_index("-1"),
        s()
            .after()
            .background_color(Color::BackgroundDark2_60)
            .bottom("0")
            .content(r#""""#)
            .left("0")
            .position(CssPosition::Absolute)
            .right("0")
            .top("0")
            .z_index("1"),
        image_url.map(|image_url| {
            img![
                C!["background-image"],
                s()
                    .display(CssDisplay::Block)
                    .filter("blur(5px)")
                    .height(pc(100))
                    .raw(r#"object-fit: cover;"#)
                    .raw(r#"object-position: center;"#)
                    .opacity("0.9")
                    .width(pc(100)),
                attrs!{
                    At::Src => image_url,
                    At::Alt => " ",
                }
            ]
        })
    ]
}

#[view]
fn meta_preview_info(selected_meta_preview: &MetaItemPreview) -> Node<Msg> {
    div![
        C!["meta-info-container"],
        s()
            .align_self(CssAlignSelf::Stretch)
            .flex("1")
            .overflow_y(CssOverflowY::Auto)
            .padding("0 2rem"),
        s()
            .not(":hover")
            .raw("scrollbar-color: transparent transparent;"),
        selected_meta_preview.logo.as_ref().map(|logo| {
            img![
                C!["logo"],
                s()
                    .background_color(Color::SurfaceDark5_10)
                    .width(pc(100))
                    .raw(r#"object-fit: contain;"#)
                    .raw(r#"object-position: center;"#)
                    .display(CssDisplay::Block)
                    .height(rem(8))
                    .margin("2rem 0")
                    .max_width(pc(100)),
                attrs!{
                    At::Src => logo,
                    At::Alt => " ",
                },
            ]
        }),
        div![
            C!["runtime-release-info-container"],
            s()
                .justify_content("space-evenly")
                .align_items(CssAlignItems::Center)
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .flex_wrap(CssFlexWrap::Wrap)
                .margin_top(rem(1)),
            selected_meta_preview.runtime.as_ref().map(|runtime| {
                div![
                    C!["runtime-label"],
                    s()
                        .margin("1rem 0.4rem")
                        .color(Color::SurfaceLight5_90)
                        .flex("0 1 auto")
                        .font_size(rem(1.4)),
                    runtime,
                ]
            }),
            selected_meta_preview.release_info.as_ref().map(|release_info| {
                div![
                    C!["release-info-label"],
                    s()
                        .margin("1rem 0.4rem")
                        .color(Color::SurfaceLight5_90)
                        .flex("0 1 auto")
                        .font_size(rem(1.4)),
                    release_info,
                ]
            }),
        ],
        div![
            C!["name-container"],
            s()
                .color(Color::SurfaceLight5_90)
                .font_size(rem(1.7))
                .margin_top(rem(1)),
            &selected_meta_preview.name,
        ],
        selected_meta_preview.description.as_ref().map(|description| {
            div![
                C!["description-container"],
                s()
                    .max_height("none")
                    .color(Color::SurfaceLight5_90)
                    .font_size(rem(1.1))
                    .line_height(em(1.5))
                    .margin_top(rem(1)),
                description,
            ]
        }),
    ]
}

#[view]
fn meta_preview_buttons(selected_meta_preview: &MetaItemPreview) -> Node<Msg> {
    div![
        C!["action-buttons-container"],
        s()
            .justify_content("space-evenly")
            .padding("0")
            .align_self(CssAlignSelf::Stretch)
            .display(CssDisplay::Flex)
            .flex(CssFlex::None)
            .flex_direction(CssFlexDirection::Row)
            .flex_wrap(CssFlexWrap::Wrap)
            .max_height(rem(10)),
        action_button("Add to library", "0 0 1264 1024", "ic_addlib", add_to_library_paths()),
        IF!(not(selected_meta_preview.trailer_streams.is_empty()) => {
            action_button("Trailer", "0 0 840 1024", "ic_movies", trailer_paths())
        }),
    ]
}

fn add_to_library_paths<Ms: 'static>() -> Vec<Node<Ms>> {
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

fn trailer_paths<Ms: 'static>() -> Vec<Node<Ms>> {
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

#[view]
fn action_button(title: &str, view_box: &str, icon: &str, paths: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        C!["action-button", "action-button-container", "button-container"],
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
        on_click_not_implemented(),
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
        pagination_input(catalog),
    ]
}

#[view]
fn pagination_input(catalog: &CatalogWithFilters<MetaItemPreview>) -> Node<Msg> {
    div![
        C!["pagination-input", "pagination-input-container"],
        s()
            .flex(CssFlex::None)
            .height(rem(3.5))
            .margin_left(rem(1.5))
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row),
        pagination_prev_button(catalog.selectable.prev_page.as_ref().map(|page| &page.request)),
        pagination_label(catalog.selected.as_ref()),
        pagination_next_button(catalog.selectable.next_page.as_ref().map(|page| &page.request)),
    ]
}

#[view]
fn pagination_label(catalog_selected: Option<&CatalogWithFiltersSelected>) -> Node<Msg> {
    let page_number = catalog_selected.and_then(|selected| {
        Some(selected.request.path.get_extra_first_value(SKIP_EXTRA_NAME)?.parse::<usize>().ok()? / CATALOG_PAGE_SIZE)
    }).unwrap_or_default() + 1;
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
fn pagination_prev_button(previous_page_request: Option<&ResourceRequest>) -> Node<Msg> {
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
            ev(Ev::Click, move |_| Msg::SendResourceRequest(request.clone()))
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
fn pagination_next_button(next_page_request: Option<&ResourceRequest>) -> Node<Msg> {
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
            ev(Ev::Click, move |_| Msg::SendResourceRequest(request.clone()))
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
fn meta_items(
    content: &Loadable<Vec<MetaItemPreview>, ResourceError>,
    selected_meta_preview: Option<&MetaItemPreview>,
    root_base_url: &Url,
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
                .map(|meta_preview| meta_item(meta_preview, selected_meta_preview, root_base_url)),
        ],
    }
}

#[view]
fn meta_item(
    meta: &MetaItemPreview, 
    selected_meta_preview: Option<&MetaItemPreview>, 
    root_base_url: &Url
) -> Node<Msg> {
    let square = matches!(meta.poster_shape, PosterShape::Square);
    let selected = selected_meta_preview.map(|meta| &meta.id) == Some(&meta.id);
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
        IF!(selected => {
            s()
                .background_color(Color::BackgroundLight3)
                .transition("background-color 100ms ease-out")
        }),
        attrs!{
            At::TabIndex => 0,
            At::Title => meta.name,
            At::Href => RootUrls::new(root_base_url).detail_urls().without_video_id(&meta.r#type, &meta.id),
        },
        IF!(not(selected) => {
            let selected_meta = meta.clone();
            ev(Ev::Click, move |event| {
                event.prevent_default();
                event.stop_propagation();
                Msg::SelectMetaItemPreview(selected_meta)
            })
        }),
        poster_container(&meta.poster, square, selected),
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
        IF!(selected => play_icon()),
    ]
}

#[view]
fn play_icon() -> Node<Msg> {
    div![
        C!["play-icon-layer"],
        s()
            .bottom("30%")
            .left("0")
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Absolute)
            .right("0")
            .top("30%")
            .z_index("-2"),
        svg![
            C!["play-icon"],
            s()
                .display(CssDisplay::Block)
                .filter("drop-shadow(0 0 0.5rem hsl(243,24.4%,21%))")
                .height(pc(100))
                .width(pc(100))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => "0 0 100 100",
            },
            circle![
                C!["background"],
                s()
                    .fill(Color::Accent4_90),
                attrs!{
                    At::Cx => 50,
                    At::Cy => 50,
                    At::R => 50,
                },
            ],
            svg![
                C!["icon"],
                s()
                    .fill(Color::SurfaceLight5_90)
                    .overflow(CssOverflow::Visible),
                attrs!{
                    At::X => 0,
                    At::Y => 25,
                    At::Width => 100,
                    At::Height => 50,
                    At::ViewBox => "0 0 37.14 32", 
                },
                path![
                    attrs!{
                        At::D => "M 9.14,0 37.14,16 9.14,32 Z",
                    },
                ],
            ]
        ],
    ]
}
