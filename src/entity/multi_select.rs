use itertools::Itertools;
use seed::{prelude::*, *};
use std::fmt::Debug;
use wasm_bindgen::JsCast;
use seed_style::{px, em, pc, rem, Style};
use seed_style::*;
use crate::styles::themes::{Color, get_color_value};

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

pub enum Msg {
    ToggleMenu,
    ItemClicked(GroupId, GroupItemId),
}

// @TODO: remove after Msg::ItemClicked refactor
#[allow(clippy::collapsible_if)]
pub fn update<T: 'static + Debug, ParentMsg>(
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
            C![
                "dropdown",
                "select-input-container",
                "multiselect-container",
                "popup-container",
                "label-container",
                "button-container",
                IF!(model.opened => "active"),
            ],
            s()
                .display(CssDisplay::Flex)
                .flex_direction(CssFlexDirection::Row)
                .align_items(CssAlignItems::Center)
                .padding("0 1rem")
                .background_color(Color::BackgroundLighter),
            s()
                .flex_grow("0")
                .flex_shrink("1")
                .flex_basis("15rem")
                .height(rem(3))
                .margin_right(rem(1)),
            s()
                .position(CssPosition::Relative)
                .overflow(CssOverflow::Visible),
            IF!(model.opened => {
                s()
                    .background_color(Color::SurfaceLight)
            }),
            attrs! {
                At::TabIndex => 0,
            },
            ev(Ev::Click, |_| Msg::ToggleMenu),
            div![
                C!["label"],
                s()
                    .flex("1")
                    .max_height(rem(2.4))
                    .color(Color::SurfaceLighter),
                IF!(model.opened => {
                    s()
                        .color(Color::BackgroundDarker)
                }),
                selected_items.iter().map(|item| &item.label).join(", "),
            ],
            svg![
                C!["icon"],
                s()
                    .flex(CssFlex::None)
                    .width(rem(1))
                    .height(rem(1))
                    .margin_left(rem(1))
                    .fill(Color::SurfaceLighter),
                IF!(model.opened => {
                    s()
                        .fill(Color::BackgroundDarker)
                }),
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
                    C![MENU_CLASS, "menu-direction-bottom",],
                    IF!(MENU_CLASS == "popup-menu-container" => {
                        s()
                            .width(pc(100))
                            .position(CssPosition::Absolute)
                            .right("0")
                            .z_index("1")
                            .overflow(CssOverflow::Visible)
                            .box_shadow(format!(
                                "0 1.35rem 2.7rem {}, 0 1.1rem 0.85rem {}", 
                                get_color_value(Color::BackgroundDarker40), 
                                get_color_value(Color::BackgroundDarker20)).as_str())
                            .cursor(CssCursor::Auto)
                            .top("100%")
                    }),
                    attrs! {
                        At::TabIndex => 0,
                    },
                    ev(Ev::Blur, |_| Msg::ToggleMenu),
                    div![
                        C!["multiselect-menu-container"],
                        s()
                            .max_height("calc(3.2rem * 7)")
                            .overflow(CssOverflow::Auto),
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
        C![
            "option-container",
            "button-container",
            IF!(item.selected => "selected"),
        ],
        s()
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .align_items(CssAlignItems::Center)
            .padding(rem(1))
            .background_color(Color::BackgroundLighter),
        IF!(item.selected => {
            s()
                .background_color(Color::SurfaceDarker)
        }),
        s()
            .hover()
            .background_color(Color::SurfaceDark),
        s()
            .focus()
            .background_color(Color::SurfaceDark),
        attrs! {
            At::Title => item.label,
        },
        ev(Ev::Click, {
            let group_id = group_id.to_owned();
            let item_id = item.id.clone();
            move |_| Msg::ItemClicked(group_id, item_id)
        }),
        div![
            C!["label"],
            s()
                .flex("1")
                .max_height(rem(4.8))
                .color(Color::SurfaceLighter),
            &item.label,
        ],
        svg![
            C!["icon"],
            s()
                .flex(CssFlex::None)
                .display(CssDisplay::None)
                .width(rem(1))
                .height(rem(1))
                .margin_left(rem(1))
                .fill(Color::SurfaceLighter),
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
