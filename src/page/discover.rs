use crate::{entity::multi_select, Context, PageId, Actions, Urls as RootUrls};
use enclose::enc;
use seed::{prelude::*, *};
use std::rc::Rc;
use stremio_core::state_types::{
    Action, ActionLoad, CatalogEntry, CatalogError, Internal, Loadable, Msg as CoreMsg, TypeEntry,
};
use stremio_core::types::MetaPreview;
use stremio_core::types::{
    addons::{ResourceRef, ResourceRequest, ResourceResponse},
    PosterShape,
};
use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use crate::styles::{self, themes::{Color, Breakpoint}, global};

mod catalog_selector;
mod extra_prop_selector;
mod type_selector;

type MetaPreviewId = String;
// @TODO add into stremio-core?
type ExtraPropOption = String;

const DEFAULT_CATALOG: &str = "top";
const DEFAULT_TYPE: &str = "movie";
const BASE: &str = "https://v3-cinemeta.strem.io/manifest.json";
const RESOURCE: &str = "catalog";

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
        [base, path] => path
            .parse()
            .map_err(|error| error!(error))
            .map(|path| ResourceRequest {
                base: base.to_string(),
                path,
            })
            .ok(),
        _ => None,
    };

    load_catalog(resource_request, orders);

    model.get_or_insert_with(move || Model {
        base_url,
        _core_msg_sub_handle: orders.subscribe_with_handle(Msg::CoreMsg),
        type_selector_model: type_selector::init(),
        catalog_selector_model: catalog_selector::init(),
        extra_prop_selector_model: extra_prop_selector::init(),
        selected_meta_preview_id: None,
    });
    Some(PageId::Discover)
}

fn load_catalog(resource_request: Option<ResourceRequest>, orders: &mut impl Orders<Msg>) {
    orders.notify(Actions::UpdateCoreModel(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::CatalogFiltered(resource_request.unwrap_or_else(default_resource_request)),
    )))));
}

pub fn default_resource_request() -> ResourceRequest {
    ResourceRequest::new(
        BASE,
        ResourceRef::without_extra(RESOURCE, DEFAULT_TYPE, DEFAULT_CATALOG),
    )
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    _core_msg_sub_handle: SubHandle,
    selected_meta_preview_id: Option<MetaPreviewId>,
    type_selector_model: type_selector::Model,
    catalog_selector_model: catalog_selector::Model,
    extra_prop_selector_model: extra_prop_selector::Model,
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
            .add_hash_path_part(&res_req.base)
            .add_hash_path_part(res_req.path.to_string())
    }
}

// ------ ------
//    Update
// ------ ------

// @TODO box large fields?
#[allow(clippy::pub_enum_variant_names, clippy::large_enum_variant)]
pub enum Msg {
    CoreMsg(Rc<CoreMsg>),
    MetaPreviewClicked(MetaPreview),
    TypeSelectorMsg(type_selector::Msg),
    TypeSelectorChanged(Vec<multi_select::Group<TypeEntry>>),
    CatalogSelectorMsg(catalog_selector::Msg),
    CatalogSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    ExtraPropSelectorMsg(extra_prop_selector::Msg),
    ExtraPropSelectorChanged(Vec<multi_select::Group<ExtraPropOption>>),
}

pub fn update(msg: Msg, model: &mut Model, context: &mut Context, orders: &mut impl Orders<Msg>) {
    let catalog = &context.core_model.catalog;

    match msg {
        Msg::CoreMsg(core_msg) => {
            if let CoreMsg::Internal(Internal::AddonResponse(_, result)) = core_msg.as_ref() {
                if let Ok(ResourceResponse::Metas { metas }) = result.as_ref() {
                    model.selected_meta_preview_id = metas.first().map(|meta| meta.id.clone());
                }
            }
        }
        Msg::MetaPreviewClicked(meta_preview) => {
            if model.selected_meta_preview_id.as_ref() == Some(&meta_preview.id) {
                let id = &meta_preview.id;
                let type_name = &meta_preview.type_name;

                let detail_urls = RootUrls::new(&context.root_base_url).detail_urls();

                orders.request_url(if meta_preview.type_name == "movie" {
                    detail_urls.with_video_id(type_name, id, id)
                } else {
                    detail_urls.without_video_id(type_name, id)
                });
            } else {
                model.selected_meta_preview_id = Some(meta_preview.id);
            }
        }

        // ------ TypeSelector  ------
        Msg::TypeSelectorMsg(msg) => {
            let msg_to_parent = type_selector::update(
                msg,
                &mut model.type_selector_model,
                &mut orders.proxy(Msg::TypeSelectorMsg),
                type_selector::groups(&catalog.types),
                Msg::TypeSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::TypeSelectorChanged(groups_with_selected_items) => {
            let res_req = type_selector::resource_request(groups_with_selected_items);
            orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        }

        // ------ CatalogSelector  ------
        Msg::CatalogSelectorMsg(msg) => {
            let msg_to_parent = catalog_selector::update(
                msg,
                &mut model.catalog_selector_model,
                &mut orders.proxy(Msg::CatalogSelectorMsg),
                catalog_selector::groups(&catalog.catalogs, &catalog.selected),
                Msg::CatalogSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::CatalogSelectorChanged(groups_with_selected_items) => {
            let res_req = catalog_selector::resource_request(groups_with_selected_items);
            orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
        }

        // ------ ExtraPropSelector  ------
        Msg::ExtraPropSelectorMsg(msg) => {
            let msg_to_parent = extra_prop_selector::update(
                msg,
                &mut model.extra_prop_selector_model,
                &mut orders.proxy(Msg::ExtraPropSelectorMsg),
                extra_prop_selector::groups(&catalog.selectable_extra, &catalog.selected),
                Msg::ExtraPropSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::ExtraPropSelectorChanged(groups_with_selected_items) => {
            if let Some(res_req) =
                extra_prop_selector::resource_request(groups_with_selected_items, &catalog.selected)
            {
                orders.request_url(Urls::new(&model.base_url).res_req(&res_req));
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, context: &Context) -> Node<Msg> {
    let catalog = &context.core_model.catalog;

    div![
        C!["discover-container"],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Column)
            .width(pc(100))
            .height(pc(100))
            .background_color(Color::Background),
        div![
            C!["discover-content"],
            s()
                .flex("1")
                .align_self(CssAlignSelf::Stretch)
                .display(CssDisplay::Grid)
                .grid_template_columns("1fr 28rem")
                .grid_template_rows("7rem 1fr")
                .grid_template_areas(r#""controls-area meta-preview-area" "catalog-content-area meta-preview-area""#),
            s()
                .only_and_below(Breakpoint::Minimum)
                .grid_template_columns("1fr")
                .grid_template_rows("fit-content(19rem) 1fr")
                .grid_template_areas(r#""controls-area" "catalog-content-area""#),
            div![
                C!["controls-container"],
                s()
                    .grid_area("controls-area")
                    .display(CssDisplay::Flex)
                    .flex_direction(CssFlexDirection::Row)
                    .margin("2rem 0")
                    .padding("0 2rem")
                    .overflow(CssOverflow::Visible),
                // type selector
                type_selector::view(
                    &model.type_selector_model,
                    &type_selector::groups(&catalog.types)
                )
                .map_msg(Msg::TypeSelectorMsg),
                // catalog selector
                catalog_selector::view(
                    &model.catalog_selector_model,
                    &catalog_selector::groups(&catalog.catalogs, &catalog.selected)
                )
                .map_msg(Msg::CatalogSelectorMsg),
                // extra prop selector
                extra_prop_selector::view(
                    &model.extra_prop_selector_model,
                    &extra_prop_selector::groups(&catalog.selectable_extra, &catalog.selected)
                )
                .map_msg(Msg::ExtraPropSelectorMsg),
                // reset button
                view_reset_button(&model.base_url),
            ],
            div![
                C!["catalog-content-container"],
                s()
                    .grid_area("catalog-content-area")
                    .margin_right(rem(2)),
                s()
                    .only_and_below(Breakpoint::Minimum)
                    .margin_right("0"),
                view_content(
                    &context.core_model.catalog.content,
                    model.selected_meta_preview_id.as_ref()
                ),
            ]
        ],
    ]
}

fn view_reset_button(base_url: &Url) -> Node<Msg> {
    a![
        s()
            .width(px(100))
            .padding("8px 20px")
            .cursor(CssCursor::Pointer)
            .display(CssDisplay::InlineBlock)
            .margin(px(5)),
        attrs! {
            At::Href => Urls::new(base_url).root()
        },
        "Reset",
    ]
}

fn view_content(
    content: &Loadable<Vec<MetaPreview>, CatalogError>,
    selected_meta_preview_id: Option<&MetaPreviewId>,
) -> Node<Msg> {
    let message_container_style = s()
        .padding("0 2rem")
        .font_size(rem(2))
        .color(Color::SurfaceLighter);

    match content {
        Loadable::Err(catalog_error) => {
            div![C!["message-container",], message_container_style, format!("{:#?}", catalog_error)]
        }
        Loadable::Loading => div![C!["message-container",], message_container_style, "Loading"],
        Loadable::Ready(meta_previews) if meta_previews.is_empty() => empty![],
        Loadable::Ready(meta_previews) => div![
            C!["meta-items-container",],
            s()
                .display(CssDisplay::Grid)
                .max_height(pc(100))
                .grid_auto_rows("max-content")
                .grid_gap(rem(1.5))
                .align_items(CssAlignItems::Center)
                .padding("0 2rem")
                .overflow_y(CssOverflowY::Auto),
            s()
                .only_and_above(Breakpoint::XXLarge)
                .grid_template_columns("repeat(8, 1fr)"),
            s()
                .only_and_below(Breakpoint::XLarge)
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
                .only_and_below(Breakpoint::Minimum)
                .grid_template_columns("repeat(5, 1fr)"),
            meta_previews
                .iter()
                .map(|meta_preview| view_meta_preview(meta_preview, selected_meta_preview_id))
                .collect::<Vec<_>>()
        ],
    }
}

fn view_meta_preview(
    meta_preview: &MetaPreview,
    selected_meta_preview_id: Option<&MetaPreviewId>,
) -> Node<Msg> {
    let poster_shape_class = match meta_preview.poster_shape {
        PosterShape::Square => "poster-shape-square",
        // @TODO correct classes
        _ => "poster-shape-poster",
    };

    let is_selected = match selected_meta_preview_id {
        Some(selected_meta_preview_id) => selected_meta_preview_id == &meta_preview.id,
        None => false,
    };

    div![
        C![
            "meta-item",
            "meta-item-container",
            poster_shape_class,
            "button-container",
            IF!(is_selected => "selected"),
        ],
        // @TODO: Rewrite styles for `meta-item-container` to Rust
        s()
            .position(CssPosition::Relative)
            .overflow(CssOverflow::Visible),
        styles::button_container(),
        IF!(is_selected => {
            s()
                .after()
                .outline_width(format!("calc(1.5 * {})", global::FOCUS_OUTLINE_SIZE).as_str())
                .raw(format!("outline-offset: calc(-1.5 * {});", global::FOCUS_OUTLINE_SIZE).as_str())
        }),
        attrs! {
            At::TabIndex => 0,
            At::Title => &meta_preview.name,
        },
        ev(
            Ev::Click,
            enc!((meta_preview) move |_| Msg::MetaPreviewClicked(meta_preview))
        ),
        div![
            C!["poster-container",],
            s()
                .position(CssPosition::Relative)
                .z_index("0")
                .background_color(Color::BackgroundLight),
            match meta_preview.poster_shape {
                PosterShape::Square => {
                    s()
                        .padding_top(pc(100))
                }
                PosterShape::Landscape => {
                    s()
                        .padding_top(format!("calc(100% * {})", styles::global::LANDSCAPE_SHAPE_RATIO).as_str())
                }
                _ => {
                    s()
                        .padding_top(format!("calc(100% * {})", styles::global::POSTER_SHAPE_RATIO).as_str())
                }
            },
            div![
                C!["poster-image-layer",], 
                s()
                    .position(CssPosition::Absolute)
                    .top("0")
                    .right("0")
                    .bottom("0")
                    .left("0")
                    .z_index("-3")
                    .display(CssDisplay::Flex)
                    .flex_direction(CssFlexDirection::Row)
                    .align_items(CssAlignItems::Center)
                    .justify_content(CssJustifyContent::Center),
                view_poster(&meta_preview.poster),
            ],
        ],
        div![
            C!["title-bar-container",],
            s()
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .align_items(CssAlignItems::Center)
                .justify_content(CssJustifyContent::FlexEnd)
                .height(rem(2.8))
                .background_color(Color::BackgroundLight)
                .overflow(CssOverflow::Visible),
            div![
                C!["title-label",], 
                s()
                    .flex("1")
                    .max_height(rem(2.4))
                    .padding_left(rem(0.5))
                    .color(Color::SurfaceLighter),
                s()
                    .only_child()
                    .padding_right(rem(0.5)),
                &meta_preview.name
            ]
        ],
    ]
}

fn view_poster(poster: &Option<String>) -> Node<Msg> {
    // @TODO Show placeholder image also if poster_url is present but can't be loaded?
    match poster {
        Some(poster_url) => img![
            C!["poster-image",],
            s()
                .flex(CssFlex::None)
                .width(pc(100))
                .height(pc(100))
                .raw("object-position: center;")
                .raw("object-fit: cover;"),
            attrs! {
                At::Src => poster_url,
            }
        ],
        None => svg![
            C!["placeholder-icon",],
            s()
                .flex(CssFlex::None)
                .width(pc(100))
                .height(pc(50))
                .fill(Color::SurfaceLight20),
            attrs! {
                At::ViewBox => "0 0 1125 1024",
                "icon" => "ic_series",
            },
            path![attrs! {
                At::D => "M1089.958 239.134c-16.353-10.225-36.218-16.289-57.499-16.289-2.977 0-5.926 0.119-8.843 0.351l0.385-0.025h-384.602c-2.584 0.543-5.552 0.854-8.594 0.854-7.913 0-15.335-2.105-21.736-5.785l0.212 0.112 94.569-99.689c21.384-22.588 42.767-45.176 63.849-68.066 11.746-12.951 16.866-27.407 3.012-41.562s-27.106-9.035-39.755 3.614c-3.975 3.53-7.614 7.168-11.028 11.011l-0.116 0.133c-46.381 48.791-93.064 96.678-138.842 146.974-12.047 12.951-20.48 16.565-33.129 0s-25.6-27.106-38.249-40.358l-113.845-117.459c-11.144-12.047-24.395-18.673-38.852-6.024-5.844 5.002-9.524 12.387-9.524 20.631s3.68 15.628 9.488 20.6l0.037 0.031c4.819 5.722 9.939 11.144 15.059 16.565 43.671 45.478 87.040 90.353 130.409 137.035 4.518 5.12 14.758 9.336 10.842 17.468s-13.553 3.614-20.781 3.614h-390.626c-70.174 0.602-101.798 32.527-101.798 102.701v596.329c0 71.981 30.118 102.099 101.496 102.099h922.504c0.033 0 0.071 0 0.11 0 14.016 0 27.726-1.315 41.011-3.829l-1.365 0.215c34.573-7.715 60.059-38.052 60.235-74.371l0-0.020q0-321.656 0-643.012c0.020-0.645 0.032-1.402 0.032-2.163 0-25.859-13.467-48.573-33.77-61.511l-0.295-0.176zM832.151 860.16c-0.171 39.458-32.197 71.379-71.679 71.379-0 0-0-0-0.001-0l-589.101 0c-39.421 0-71.379-31.957-71.379-71.379h-0v-478.569c-0-0-0-0-0-0.001 0-39.482 31.921-71.508 71.363-71.679l0.016-0h589.101c39.519 0.17 71.51 32.161 71.68 71.664l0 0.016zM980.932 595.125c-30.393-0.468-55.009-24.558-56.316-54.695l-0.004-0.119c-0-0.001-0-0.002-0-0.003 0-29.895 24.064-54.169 53.878-54.509l0.032-0c1.283-0.116 2.775-0.182 4.283-0.182 27.944 0 50.598 22.653 50.598 50.598 0 0.911-0.024 1.817-0.072 2.717l0.005-0.126c0.009 0.367 0.014 0.8 0.014 1.234 0 29.809-23.664 54.090-53.231 55.084l-0.091 0.002zM980.932 422.852c-0.089 0.001-0.195 0.001-0.3 0.001-30.439 0-55.115-24.676-55.115-55.115s24.676-55.115 55.115-55.115c30.439 0 55.115 24.676 55.115 55.115 0 0.106-0 0.211-0.001 0.317l0-0.016c0 0 0 0.001 0 0.001 0 29.608-24.002 53.609-53.609 53.609-0.106 0-0.212-0-0.317-0.001l0.016 0h-2.409z"
            }]
        ],
    }
}
