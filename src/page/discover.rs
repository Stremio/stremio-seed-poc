use crate::{entity::multi_select, route::Route, GMsg, SharedModel};
use seed::{prelude::*, *};
use std::rc::Rc;
use enclose::enc;
use stremio_core::state_types::{
    Action, ActionLoad, CatalogEntry, CatalogError, Internal, Loadable, Msg as CoreMsg, TypeEntry,
};
use stremio_core::types::MetaPreview;
use stremio_core::types::{
    addons::{ResourceRef, ResourceRequest, ResourceResponse},
    PosterShape,
};

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
    shared: SharedModel,
    selected_meta_preview_id: Option<MetaPreviewId>,
    type_selector_model: type_selector::Model,
    catalog_selector_model: catalog_selector::Model,
    extra_prop_selector_model: extra_prop_selector::Model,
}

impl Model {
    pub fn shared(&mut self) -> &mut SharedModel {
        &mut self.shared
    }
}

impl From<Model> for SharedModel {
    fn from(model: Model) -> Self {
        model.shared
    }
}

// ------ ------
//     Init
// ------ ------

pub fn init(
    shared: SharedModel,
    resource_request: Option<ResourceRequest>,
    orders: &mut impl Orders<Msg>,
) -> Model {
    load_catalog(resource_request, orders);
    Model {
        shared,
        type_selector_model: type_selector::init(),
        catalog_selector_model: catalog_selector::init(),
        extra_prop_selector_model: extra_prop_selector::init(),
        selected_meta_preview_id: None,
    }
}

fn load_catalog(resource_request: Option<ResourceRequest>, orders: &mut impl Orders<Msg>) {
    // @TODO try to remove `Clone` requirement from Seed or add it into stremio-core? Implement intos, from etc.?
    orders.send_g_msg(GMsg::Core(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::CatalogFiltered(resource_request.unwrap_or_else(default_resource_request)),
    )))));
}

// ------ ------
//     Sink
// ------ ------

pub fn sink(g_msg: GMsg, model: &mut Model, orders: &mut impl Orders<Msg>) -> Option<GMsg> {
    match g_msg {
        GMsg::GoTo(Route::Discover(resource_request)) => {
            load_catalog(resource_request, orders);
            return None;
        }
        GMsg::Core(ref core_msg) => {
            if let CoreMsg::Internal(Internal::AddonResponse(_, result)) = core_msg.as_ref() {
                if let Ok(ResourceResponse::Metas { metas }) = result.as_ref() {
                    model.selected_meta_preview_id = metas.first().map(|meta| meta.id.clone());
                }
            }
        }
        _ => (),
    }
    Some(g_msg)
}

// ------ ------
//    Update
// ------ ------

// @TODO box large fields?
#[allow(clippy::pub_enum_variant_names, clippy::large_enum_variant)]
pub enum Msg {
    MetaPreviewClicked(MetaPreview),
    TypeSelectorMsg(type_selector::Msg),
    TypeSelectorChanged(Vec<multi_select::Group<TypeEntry>>),
    CatalogSelectorMsg(catalog_selector::Msg),
    CatalogSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    ExtraPropSelectorMsg(extra_prop_selector::Msg),
    ExtraPropSelectorChanged(Vec<multi_select::Group<ExtraPropOption>>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let catalog = &model.shared.core.catalog;

    match msg {
        Msg::MetaPreviewClicked(meta_preview) => {
            if model.selected_meta_preview_id.as_ref() == Some(&meta_preview.id) {
                let detail_route = Route::Detail {
                    video_id: if meta_preview.type_name == "movie" {
                        Some(meta_preview.id.clone())
                    } else {
                        None
                    },
                    type_name: meta_preview.type_name,
                    id: meta_preview.id,
                };
                orders.send_g_msg(GMsg::GoTo(detail_route));
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
            let req = type_selector::resource_request(groups_with_selected_items);
            orders.send_g_msg(GMsg::GoTo(Route::Discover(Some(req))));
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
            let req = catalog_selector::resource_request(groups_with_selected_items);
            orders.send_g_msg(GMsg::GoTo(Route::Discover(Some(req))));
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
            if let Some(req) =
                extra_prop_selector::resource_request(groups_with_selected_items, &catalog.selected)
            {
                orders.send_g_msg(GMsg::GoTo(Route::Discover(Some(req))));
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let catalog = &model.shared.core.catalog;

    div![
        C!["discover-container"],
        div![
            C!["discover-content"],
            div![
                C!["controls-container"],
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
                view_reset_button(),
            ],
            div![
                C!["catalog-content-container"],
                view_content(
                    &model.shared.core.catalog.content,
                    model.selected_meta_preview_id.as_ref()
                ),
            ]
        ],
    ]
}

fn view_reset_button() -> Node<Msg> {
    a![
        style! {
            St::Width => px(100),
            St::Padding => "8px 20px",
            St::Cursor => "pointer",
            St::Display => "inline-block",
            St::Margin => px(5),
            St::Cursor => "pointer",
        },
        attrs! {
            At::Href => Route::Discover(None).to_href()
        },
        "Reset",
    ]
}

fn view_content(
    content: &Loadable<Vec<MetaPreview>, CatalogError>,
    selected_meta_preview_id: Option<&MetaPreviewId>,
) -> Node<Msg> {
    match content {
        Loadable::Err(catalog_error) => div![
            C!["message-container",],
            format!("{:#?}", catalog_error)
        ],
        Loadable::Loading => div![C!["message-container",], "Loading"],
        Loadable::Ready(meta_previews) if meta_previews.is_empty() => empty![],
        Loadable::Ready(meta_previews) => div![
            C!["meta-items-container",],
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
        attrs! {
            At::TabIndex => 0,
            At::Title => &meta_preview.name,
        },
        ev(Ev::Click, enc!((meta_preview) move |_| Msg::MetaPreviewClicked(meta_preview))),
        div![
            C!["poster-container",],
            div![
                C!["poster-image-layer",],
                view_poster(&meta_preview.poster),
            ],
        ],
        div![
            C!["title-bar-container",],
            div![C!["title-label",], &meta_preview.name]
        ],
    ]
}

fn view_poster(poster: &Option<String>) -> Node<Msg> {
    // @TODO Show placeholder image also if poster_url is present but can't be laoded?
    match poster {
        Some(poster_url) => img![
            C!["poster-image",],
            attrs! {
                At::Src => poster_url,
            }
        ],
        None => svg![
            C!["placeholder-icon",],
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
