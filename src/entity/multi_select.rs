use itertools::Itertools;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;

const MENU_CLASS: &str = "popup-menu-container";

#[derive(Clone, Debug)]
pub struct Group<T> {
    pub id: GroupId,
    pub label: Option<String>,
    pub items: Vec<GroupItem<T>>,
    pub limit: usize,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub struct GroupItem<T> {
    pub id: GroupItemId,
    pub label: String,
    pub value: T,
    pub selected: bool,
}

// ------ ------
//     Model
// ------ ------

pub type GroupId = String;
pub type GroupItemId = String;

pub struct Model {
    id: &'static str,
    opened: bool,
}

// ------ ------
//     Init
// ------ ------

pub const fn init(id: &'static str) -> Model {
    Model { id, opened: false }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    ToggleMenu,
    ItemClicked(GroupId, GroupItemId),
    NoOp,
}

// @TODO: remove after Msg::ItemClicked refactor
#[allow(clippy::collapsible_if)]
pub fn update<T: 'static, ParentMsg>(
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
    mut groups: Vec<Group<T>>,
    on_change: impl FnOnce(Vec<Group<T>>) -> ParentMsg,
) -> Option<ParentMsg> {
    match msg {
        Msg::ToggleMenu => {
            model.opened = !model.opened;

            let selector_id = model.id;
            if model.opened {
                orders.after_next_render(move |_| {
                    document()
                        .query_selector(&format!("#{} .{}", selector_id, MENU_CLASS))
                        .unwrap()
                        .expect("menu element")
                        .dyn_into::<web_sys::HtmlElement>()
                        .expect("menu element as `HtmlElement`")
                        .focus()
                        .expect("focus menu element");
                    Msg::NoOp
                });
            }
            None
        }
        // @TODO: Refactor + comments
        Msg::ItemClicked(group_id, item_id) => {
            let first_selected_address =
                groups.iter().enumerate().find_map(|(group_index, group)| {
                    if group.id == group_id {
                        if let Some(item_index) = group.items.iter().position(|item| item.selected)
                        {
                            Some((group_index, item_index))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });

            let selected_count: usize = groups
                .iter()
                .map(|group| group.items.iter().filter(|item| item.selected).count())
                .sum();

            let group = groups.iter_mut().find(|group| {
                group.id == group_id && group.items.iter().any(|item| item.id == item_id)
            }).expect("`Group` with given `group_id`, which contains `GroupItem` with given `item_id`");

            let item = group
                .items
                .iter_mut()
                .find(|item| item.id == item_id)
                .expect("`GroupItem` with given `item_id`");

            let changed = if item.selected {
                if !group.required || selected_count > 1 {
                    item.selected = false;
                    true
                } else {
                    false
                }
            } else {
                if selected_count < group.limit {
                    item.selected = true;
                    true
                } else {
                    if let Some((first_selected_group_index, first_selected_item_index)) =
                        first_selected_address
                    {
                        item.selected = true;
                        groups
                            .get_mut(first_selected_group_index)
                            .unwrap()
                            .items
                            .get_mut(first_selected_item_index)
                            .unwrap()
                            .selected = false;
                        true
                    } else {
                        false
                    }
                }
            };

            if changed {
                let groups_with_selected_items = groups
                    .into_iter()
                    .filter_map(|mut group| {
                        group.items = group
                            .items
                            .into_iter()
                            .filter(|item| item.selected)
                            .collect();
                        if group.items.is_empty() {
                            None
                        } else {
                            Some(group)
                        }
                    })
                    .collect::<Vec<_>>();
                Some(on_change(groups_with_selected_items))
            } else {
                None
            }
        }
        Msg::NoOp => None,
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<T: Clone>(model: &Model, groups: &[Group<T>]) -> Node<Msg> {
    let selected_items = groups
        .iter()
        .flat_map(|group| group.items.iter().filter(|item| item.selected))
        .collect::<Vec<_>>();

    if selected_items.is_empty() {
        empty![]
    } else {
        div![
            id!(model.id),
            class![
                "select-input-container",
                "multiselect-container",
                "popup-container",
                "label-container",
                "button-container",
                "active" => model.opened,
            ],
            attrs! {
                At::TabIndex => 0,
            },
            simple_ev(Ev::Click, Msg::ToggleMenu),
            div![
                class!["label"],
                selected_items.iter().map(|item| &item.label).join(", "),
            ],
            svg![
                class!["icon"],
                attrs! {
                    At::ViewBox => "0 0 1731 1024",
                    "icon" => "ic_arrow_down",
                },
                path![attrs! {
                    At::D => "M1674.541 54.212c-35.054-33.866-82.855-54.734-135.529-54.734s-100.475 20.868-135.585 54.788l0.056-0.054-538.202 523.144-539.708-523.144c-34.993-34.004-82.813-54.97-135.529-54.97s-100.536 20.966-135.576 55.015l0.046-0.045c-34.583 32.979-56.087 79.409-56.087 130.861s21.504 97.882 56.015 130.793l0.072 0.068 675.84 653.854c35.054 33.866 82.855 54.734 135.529 54.734s100.475-20.868 135.585-54.788l-0.056 0.054 673.129-653.854c34.583-32.979 56.087-79.409 56.087-130.861s-21.504-97.882-56.015-130.793l-0.072-0.068z"
                }]
            ],
            if model.opened {
                div![
                    class![MENU_CLASS, "menu-direction-bottom",],
                    attrs! {
                        At::TabIndex => 0,
                    },
                    simple_ev(Ev::Blur, Msg::ToggleMenu),
                    div![
                        class!["multiselect-menu-container"],
                        groups.iter().map(view_group).collect::<Vec<_>>()
                    ]
                ]
            } else {
                empty![]
            }
        ]
    }
}

pub fn view_group<T: Clone>(group: &Group<T>) -> Node<Msg> {
    div![
        // @TODO remove?

        //        match &group.label {
        //            Some(label) => {
        //                div![
        //                    label,
        //                ]
        //            }
        //            None => empty![],
        //        },
        group
            .items
            .iter()
            .map(|item| view_group_item(&group.id, item))
            .collect::<Vec<_>>()
    ]
}

pub fn view_group_item<T: Clone>(group_id: &str, item: &GroupItem<T>) -> Node<Msg> {
    div![
        class![
            "option-container",
            "button-container",
            "selected" => item.selected,
        ],
        simple_ev(
            Ev::Click,
            Msg::ItemClicked(group_id.to_owned(), item.id.clone())
        ),
        div![class!["label"], item.label,],
        svg![
            class!["icon"],
            attrs! {
                At::ViewBox => "0 0 1331 1024",
                "icon" => "ic_check",
            },
            path![attrs! {
                At::D => "M545.129 1024c-40.334-0.026-76.847-16.363-103.306-42.769l-398.755-397.551c-24.752-26.158-39.97-61.56-39.97-100.516 0-80.839 65.533-146.372 146.372-146.372 38.806 0 74.085 15.101 100.281 39.748l-0.075-0.070 288.226 286.118 536.395-612.593c27.002-30.81 66.432-50.158 110.381-50.158 80.929 0 146.535 65.606 146.535 146.535 0 36.98-13.698 70.761-36.298 96.544l0.144-0.168-639.699 731.256c-25.909 29.451-63.15 48.401-104.838 49.987l-0.272 0.008z"
            }]
        ]
    ]
}
