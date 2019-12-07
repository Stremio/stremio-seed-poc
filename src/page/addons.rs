use seed::{prelude::*, *};
use crate::{entity::multi_select, route::Route, SharedModel};
use std::rc::Rc;
use std::convert::TryFrom;
use futures::future::Future;
use stremio_core::state_types::{
    Action, ActionLoad, CatalogEntry, CatalogError, Loadable, Msg as CoreMsg, TypeEntry, Update,
};
use stremio_core::types::{addons::{ResourceRequest, ResourceRef}, PosterShape};
use stremio_core::types::addons::DescriptorPreview;

mod catalog_selector;
mod type_selector;

const DEFAULT_CATALOG: &str = "thirdparty";
const DEFAULT_TYPE: &str = "movie";

pub fn default_resource_request() -> ResourceRequest {
    ResourceRequest {
        base: "https://v3-cinemeta.strem.io/manifest.json".to_owned(),
        path: ResourceRef::without_extra("addon_catalog", DEFAULT_TYPE, DEFAULT_CATALOG),
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    shared: SharedModel,
    catalog_selector_model: catalog_selector::Model,
    type_selector_model: type_selector::Model,
    search_query: String,
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
    orders.send_msg(
        // @TODO try to remove `Clone` requirement from Seed or add it into stremi-core? Implement intos, from etc.?
        // @TODO select the first preview on Load
        Msg::Core(Rc::new(CoreMsg::Action(Action::Load(
            ActionLoad::CatalogFiltered(resource_request.unwrap_or_else(|| default_resource_request())),
        )))),
    );

    Model {
        shared,
        catalog_selector_model: catalog_selector::init(),
        type_selector_model: type_selector::init(),
        search_query: String::new(),
    }
}

// ------ ------
//    Update
// ------ ------

#[allow(clippy::pub_enum_variant_names)]
#[derive(Clone)]
pub enum Msg {
    Core(Rc<CoreMsg>),
    CoreError(Rc<CoreMsg>),
    CatalogSelectorMsg(catalog_selector::Msg),
    CatalogSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    TypeSelectorMsg(type_selector::Msg),
    TypeSelectorChanged(Vec<multi_select::Group<CatalogEntry>>),
    SearchQueryChanged(String),
    AddAddonButtonClicked,
    UninstallAddonButtonClicked(DescriptorPreview),
    ShareAddonButtonClicked(DescriptorPreview),
}

fn push_resource_request(req: ResourceRequest, orders: &mut impl Orders<Msg>) {
    let route = Route::Addons(Some(req.clone()));
    let url = Url::try_from(route.to_href()).expect("`Url` from `Route::Addons`");
    seed::push_route(url);

    orders.send_msg(Msg::Core(Rc::new(CoreMsg::Action(Action::Load(
        ActionLoad::CatalogFiltered(req),
    )))));
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let catalog = &model.shared.core.addon_catalog;

    match msg {
        // @TODO: move to lib.rs? (check also other pages)
        // ------ Core  ------
        Msg::Core(core_msg) => {
            let fx = model.shared.core.update(&core_msg);

            if !fx.has_changed {
                orders.skip();
            }

            for cmd in fx.effects {
                let cmd = cmd
                    .map(|core_msg| Msg::Core(Rc::new(core_msg)))
                    .map_err(|core_msg| Msg::CoreError(Rc::new(core_msg)));
                orders.perform_cmd(cmd);
            }
        }
        Msg::CoreError(core_error) => log!("core_error", core_error),

        // ------ CatalogSelector  ------
        Msg::CatalogSelectorMsg(msg) => {
            let msg_to_parent = catalog_selector::update(
                msg,
                &mut model.catalog_selector_model,
                &mut orders.proxy(Msg::CatalogSelectorMsg),
                catalog_selector::groups(&catalog.catalogs),
                Msg::CatalogSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::CatalogSelectorChanged(groups_with_selected_items) => {
            let req = catalog_selector::resource_request(groups_with_selected_items);
            push_resource_request(req, orders)
        }

        // ------ TypeSelector  ------
        Msg::TypeSelectorMsg(msg) => {
            let msg_to_parent = type_selector::update(
                msg,
                &mut model.type_selector_model,
                &mut orders.proxy(Msg::TypeSelectorMsg),
                type_selector::groups(&catalog.catalogs, &catalog.selected),
                Msg::TypeSelectorChanged,
            );
            if let Some(msg) = msg_to_parent {
                orders.send_msg(msg);
            }
        }
        Msg::TypeSelectorChanged(groups_with_selected_items) => {
            let req = type_selector::resource_request(groups_with_selected_items);
            push_resource_request(req, orders)
        }

        Msg::SearchQueryChanged(search_query) => model.search_query = search_query,
        Msg::AddAddonButtonClicked => log!("add addon button clicked"),
        Msg::UninstallAddonButtonClicked(addon) => log!("uninstall button clicked", addon),
        Msg::ShareAddonButtonClicked(addon) => log!("share button clicked", addon),
    }
}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    let catalog = &model.shared.core.addon_catalog;

    div![
        class!["addons-container"],
        div![
            class!["addons-content"],
            div![
                class!["top-bar-container"],
                // add addon button
                view_add_addon_button(),
                // catalog selector
                catalog_selector::view(
                    &model.catalog_selector_model,
                    &catalog_selector::groups(&catalog.catalogs)
                )
                .map_message(Msg::CatalogSelectorMsg),
                // type selector
                type_selector::view(
                    &model.type_selector_model,
                    &type_selector::groups(&catalog.catalogs, &catalog.selected)
                )
                .map_message(Msg::TypeSelectorMsg),
                // search input
                view_search_input(&model.search_query),
            ],
            div![
                class!["addons-list-container"],
                view_content(&model.shared.core.addon_catalog.content),
            ]
        ],
    ]
}

fn view_add_addon_button() -> Node<Msg> {
    div![
        class![
            "add-button-container", "button-container",
        ],
        attrs!{
            At::TabIndex => 0,
        },
        simple_ev(Ev::Click, Msg::AddAddonButtonClicked),
        svg![
            class!["icon",],
            attrs! {
                At::ViewBox => "0 0 1024 1024",
                "icon" => "ic_plus",
            },
            path![attrs! {
                At::D => "M576.151 576.151h383.699c35.429 0 64.151-28.721 64.151-64.151s-28.721-64.151-64.151-64.151v-0h-383.699v-383.699c0-35.429-28.721-64.151-64.151-64.151s-64.151 28.721-64.151 64.151h-0v383.699h-383.699c-35.429 0-64.151 28.721-64.151 64.151s28.721 64.151 64.151 64.151v0h383.699v383.699c0 35.429 28.721 64.151 64.151 64.151s64.151-28.721 64.151-64.151v0z"
            }]
        ],
        div![
            class![
                "add-button-label"
            ],
            "Add addon"
        ]
    ]
}

fn view_search_input(search_query: &str) -> Node<Msg> {
    div![
        class![
            "search-bar-container",
        ],
        svg![
            class!["icon",],
            attrs! {
                At::ViewBox => "0 0 1025 1024",
                "icon" => "ic_search",
            },
            path![attrs! {
                At::D => "M1001.713 879.736c-48.791-50.899-162.334-163.84-214.438-216.546 43.772-66.969 69.909-148.918 70.174-236.956l0-0.070c-1.877-235.432-193.166-425.561-428.862-425.561-236.861 0-428.875 192.014-428.875 428.875 0 236.539 191.492 428.353 427.909 428.874l0.050 0c1.551 0.021 3.382 0.033 5.216 0.033 85.536 0 165.055-25.764 231.219-69.956l-1.518 0.954 201.487 204.499c16.379 18.259 39.94 29.789 66.201 30.117l0.058 0.001c2.034 0.171 4.401 0.269 6.791 0.269 35.32 0 65.657-21.333 78.83-51.816l0.214-0.556c5.589-10.528 8.87-23.018 8.87-36.275 0-21.857-8.921-41.631-23.32-55.878l-0.007-0.007zM429.478 730.654c-0.004 0-0.008 0-0.012 0-166.335 0-301.176-134.841-301.176-301.176 0-0.953 0.004-1.905 0.013-2.856l-0.001 0.146c0.599-165.882 135.211-300.124 301.176-300.124 166.336 0 301.178 134.842 301.178 301.178 0 0.371-0.001 0.741-0.002 1.111l0-0.057c0 0.179 0.001 0.391 0.001 0.603 0 166.335-134.841 301.176-301.176 301.176-0.106 0-0.212-0-0.318-0l0.016 0z"
            }]
        ],
        input![
            class![
                "search-input", "text-input"
            ],
            attrs!{
                At::TabIndex => 0,
                At::Placeholder => "Search addons...",
                At::Value => search_query,
            },
            input_ev(Ev::Input, Msg::SearchQueryChanged),
        ]
    ]
}

fn view_content(
    content: &Loadable<Vec<DescriptorPreview>, CatalogError>,
) -> Vec<Node<Msg>> {
    match content {
        Loadable::Err(catalog_error) => vec![div![
            class!["message-container",],
            format!("{:#?}", catalog_error)
        ]],
        Loadable::Loading => vec![div![class!["message-container",], "Loading"]],
        Loadable::Ready(addons) if addons.is_empty() => Vec::new(),
        Loadable::Ready(addons) => {
            addons
                .iter()
                .map(view_addon)
                .collect()
        },
    }
}

// ------ view addon ------

fn view_addon(
    addon: &DescriptorPreview,
) -> Node<Msg> {
    div![
        class![
            "addon-container",
            "addon",
            "button-container",
        ],
        attrs!{
            At::TabIndex => 0,
        },
        view_logo_container(&addon.manifest.logo),
        view_info_container(addon),
        view_buttons_container(addon)
    ]
}

fn view_logo_container(logo_url: &Option<String>) -> Node<Msg> {
    div![
        class![
            "logo-container"
        ],
        if let Some(logo_url) = logo_url {
            img![
                class![
                    "logo",
                ],
                attrs!{
                    At::Src => logo_url,
                }
            ]
        } else {
            svg![
                class!["icon",],
                attrs! {
                    At::ViewBox => "0 0 1043 1024",
                    "icon" => "ic_addons",
                },
                path![attrs! {
                    At::D => "M145.468 679.454c-40.056-39.454-80.715-78.908-120.471-118.664-33.431-33.129-33.129-60.235 0-90.353l132.216-129.807c5.693-5.938 12.009-11.201 18.865-15.709l0.411-0.253c23.492-15.059 41.864-7.529 48.188 18.974 0 7.228 2.711 14.758 3.614 22.287 3.801 47.788 37.399 86.785 82.050 98.612l0.773 0.174c10.296 3.123 22.128 4.92 34.381 4.92 36.485 0 69.247-15.94 91.702-41.236l0.11-0.126c24.858-21.654 40.48-53.361 40.48-88.718 0-13.746-2.361-26.941-6.701-39.201l0.254 0.822c-14.354-43.689-53.204-75.339-99.907-78.885l-0.385-0.023c-18.372-2.409-41.562 0-48.188-23.492s11.445-34.635 24.998-47.887q65.054-62.946 130.409-126.795c32.527-31.925 60.235-32.226 90.353 0 40.659 39.153 80.715 78.908 120.471 118.362 8.348 8.594 17.297 16.493 26.82 23.671l0.587 0.424c8.609 7.946 20.158 12.819 32.846 12.819 24.823 0 45.29-18.653 48.148-42.707l0.022-0.229c3.012-13.252 4.518-26.805 8.734-39.755 12.103-42.212 50.358-72.582 95.705-72.582 3.844 0 7.637 0.218 11.368 0.643l-0.456-0.042c54.982 6.832 98.119 49.867 105.048 104.211l0.062 0.598c0.139 1.948 0.218 4.221 0.218 6.512 0 45.084-30.574 83.026-72.118 94.226l-0.683 0.157c-12.348 3.915-25.299 5.722-37.948 8.433-45.779 9.638-60.235 46.984-30.118 82.824 15.265 17.569 30.806 33.587 47.177 48.718l0.409 0.373c31.925 31.925 64.452 62.946 96.075 94.871 13.698 9.715 22.53 25.511 22.53 43.369s-8.832 33.655-22.366 43.259l-0.164 0.111c-45.176 45.176-90.353 90.353-137.035 134.325-5.672 5.996-12.106 11.184-19.169 15.434l-0.408 0.227c-4.663 3.903-10.725 6.273-17.341 6.273-13.891 0-25.341-10.449-26.92-23.915l-0.012-0.127c-2.019-7.447-3.714-16.45-4.742-25.655l-0.077-0.848c-4.119-47.717-38.088-86.476-82.967-97.721l-0.76-0.161c-9.584-2.63-20.589-4.141-31.947-4.141-39.149 0-74.105 17.956-97.080 46.081l-0.178 0.225c-21.801 21.801-35.285 51.918-35.285 85.185 0 1.182 0.017 2.36 0.051 3.533l-0.004-0.172c1.534 53.671 40.587 97.786 91.776 107.115l0.685 0.104c12.649 2.409 25.901 3.313 38.249 6.626 22.588 6.325 30.118 21.685 18.372 41.864-4.976 8.015-10.653 14.937-17.116 21.035l-0.051 0.047c-44.875 44.574-90.353 90.353-135.228 133.12-10.241 14.067-26.653 23.106-45.176 23.106s-34.935-9.039-45.066-22.946l-0.111-0.159c-40.659-38.852-80.414-78.908-120.471-118.362z"
                }]
            ]
        }
    ]
}

fn view_info_container(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        class![
            "info-container"
        ],
        div![
            class![
                "name-container"
            ],
            addon.manifest.name,
        ],
        div![
            class![
                "version-container"
            ],
            format!("v.{}", addon.manifest.version),
        ],
        div![
            class![
                "types-container"
            ],
             format_addon_types(&addon.manifest.types),
        ],
        if let Some(description) = &addon.manifest.description {
            div![
                class![
                    "description-container"
                ],
                description,
            ]
        } else {
            empty![]
        }
    ]
}

fn format_addon_types(types: &[String]) -> String {
    match types.len() {
        0 => "".to_owned(),
        1 => types[0].to_owned(),
        _ => {
            let (last, rest) = types.split_last().unwrap();
            format!("{} & {}", rest.join(", "), last)
        }
    }
}

fn view_buttons_container(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        class![
            "buttons-container"
        ],
        view_uninstall_addon_button(addon),
        view_share_addon_button(addon)
    ]
}

fn view_uninstall_addon_button(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        class![
            "uninstall-button-container",
            "button-container",
        ],
        attrs!{
            At::TabIndex => -1,
        },
        simple_ev(Ev::Click, Msg::UninstallAddonButtonClicked(addon.clone())),
        div![
            class![
                "label",
            ],
            "Uninstall"
        ]
    ]
}

fn view_share_addon_button(addon: &DescriptorPreview) -> Node<Msg> {
    div![
        class![
            "share-button-container",
            "button-container",
        ],
        attrs!{
            At::TabIndex => -1,
        },
        simple_ev(Ev::Click, Msg::ShareAddonButtonClicked(addon.clone())),
        svg![
            class!["icon",],
            attrs! {
                At::ViewBox => "0 0 1024 1024",
                "icon" => "ic_share",
            },
            path![attrs! {
                At::D => "M846.005 679.454c-62.726 0.19-117.909 32.308-150.171 80.95l-0.417 0.669-295.755-96.979c2.298-11.196 3.614-24.064 3.614-37.239 0-0.038-0-0.075-0-0.113l0 0.006c0-0.039 0-0.085 0-0.132 0-29.541-6.893-57.472-19.159-82.272l0.486 1.086 221.967-143.059c42.092 37.259 97.727 60.066 158.685 60.235l0.035 0c0.81 0.010 1.768 0.016 2.726 0.016 128.794 0 233.38-103.646 234.901-232.079l0.001-0.144c0-131.737-106.794-238.532-238.532-238.532s-238.532 106.794-238.532 238.532h0c0.012 33.532 7.447 65.325 20.752 93.828l-0.573-1.367-227.087 146.372c-32.873-23.074-73.687-36.92-117.729-37.045l-0.031-0c-0.905-0.015-1.974-0.023-3.044-0.023-108.186 0-196.124 86.69-198.139 194.395l-0.003 0.189c2.017 107.893 89.956 194.583 198.142 194.583 1.070 0 2.139-0.008 3.205-0.025l-0.161 0.002c0.108 0 0.235 0 0.363 0 60.485 0 114.818-26.336 152.159-68.168l0.175-0.2 313.826 103.002c-0.004 0.448-0.006 0.976-0.006 1.506 0 98.47 79.826 178.296 178.296 178.296s178.296-79.826 178.296-178.296c0-98.468-79.823-178.293-178.29-178.296l-0-0zM923.106 851.727c0.054 1.079 0.084 2.343 0.084 3.614 0 42.748-34.654 77.402-77.402 77.402s-77.402-34.654-77.402-77.402c0-42.748 34.654-77.402 77.402-77.402 0.076 0 0.152 0 0.229 0l-0.012-0c0.455-0.010 0.99-0.015 1.527-0.015 41.12 0 74.572 32.831 75.572 73.711l0.002 0.093zM626.748 230.4c3.537-73.358 63.873-131.495 137.788-131.495s134.251 58.137 137.776 131.179l0.012 0.316c-3.537 73.358-63.873 131.495-137.788 131.495s-134.251-58.137-137.776-131.179l-0.012-0.316zM301.176 626.748c-1.34 53.35-44.907 96.087-98.456 96.087-0.54 0-1.078-0.004-1.616-0.013l0.081 0.001c-1.607 0.096-3.486 0.151-5.377 0.151-53.061 0-96.075-43.014-96.075-96.075s43.014-96.075 96.075-96.075c1.892 0 3.77 0.055 5.635 0.162l-0.258-0.012c0.459-0.008 1-0.012 1.543-0.012 53.443 0 96.943 42.568 98.445 95.648l0.003 0.139z"
            }]
        ],
        div![
            class![
                "label",
            ],
            "Share addon"
        ]
    ]
}
