use itertools::Itertools;
use seed::{prelude::*, *};
use std::fmt::Debug;
use wasm_bindgen::JsCast;
use seed_styles::{pc, em, rem};
use seed_styles::*;
use crate::styles::{self, themes::{Color, get_color_value}};

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
    let active = true;
    let title = "Select type";
    let value = "movie";
    let left_margin = true;
    div![
        C!["select-input", "label-container", "button-container", IF!(active => "active")],
        s()
            .flex("0 1 15rem")
            .height(rem(3.5))
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .padding("0 1rem")
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Relative)
            .cursor(CssCursor::Pointer),
        IF!(left_margin => s().margin_left(rem(1.5))),
        attrs!{
            At::TabIndex => 0,
            At::Title => title,
        },
        div![
            C!["label", IF!(active => "active")],
            s()
                .color(if active { Color::SurfaceLight5_90 } else { Color::SecondaryVariant1_90 })
                .flex("1")
                .font_weight("500")
                .max_height(rem(2.4)),
            value
        ],
        svg![
            C!["icon"],
            s()
                .fill(if active { Color::SurfaceLight5_90 } else { Color::SecondaryVariant1_90 })
                .flex(CssFlex::None)
                .height(rem(1))
                .margin_left(rem(1))
                .width(rem(1))
                .overflow(CssOverflow::Visible),
            attrs!{
                At::ViewBox => "0 0 1024 1024",
                At::from("icon") => "ic_arrow_thin_down",
            },
            path![
                attrs!{
                    At::D => "M14.155 314.428l463.511 465.318c8.928 8.731 21.149 14.127 34.63 14.155l0.005 0c0.103 0.001 0.225 0.001 0.348 0.001 13.437 0 25.582-5.534 34.278-14.448l0.009-0.010 462.908-463.812c8.82-9.052 14.26-21.434 14.26-35.087s-5.44-26.035-14.27-35.098l0.010 0.011c-8.905-8.816-21.115-14.308-34.607-14.456l-0.028-0c-13.572 0.165-25.802 5.779-34.629 14.751l-0.006 0.007-428.574 428.273-427.972-429.779c-8.799-8.927-21.024-14.458-34.541-14.458-0.139 0-0.278 0.001-0.417 0.002l0.021-0c-0.043-0-0.094-0-0.145-0-13.595 0-25.899 5.526-34.789 14.455l-0.002 0.002c-8.82 9.052-14.26 21.434-14.26 35.087s5.44 26.035 14.27 35.098l-0.010-0.011z",
                }
            ]
        ],
        IF!(active => menu()),
    ]
}

fn menu() -> Node<Msg> {
    div![
        C!["menu-container", "menu-direction-bottom-right"],
        s()
            .bottom("initial")
            .left("0")
            .right("initial")
            .top("100%")
            .visibility(CssVisibility::Visible)
            .width(pc(100))
            .box_shadow(format!(
                "0 1.35rem 2.7rem {}, 0 1.1rem 0.85rem {}",
                "hsla(0,0%,0%,0.4)",
                "hsla(0,0%,0%,0.2)",
            ).as_str())
            .cursor(CssCursor::Auto)
            .overflow(CssOverflow::Visible)
            .position(CssPosition::Absolute)
            .z_index("1"),
        div![
            C!["menu-container"],
            s()
                .max_height("calc(3.2rem * 7)")
                .overflow(CssOverflow::Auto),
            menu_item(),
            menu_item(),
            menu_item(),
            menu_item(),
        ]
    ]
}

fn menu_item() -> Node<Msg> {
    let selected = true;
    let title = "series";
    div![
        C!["option-container", "button-container", IF!(selected => "selected")],
        s()
            .align_items(CssAlignItems::Center)
            .background_color(Color::Background)
            .display(CssDisplay::Flex)
            .flex_direction(CssFlexDirection::Row)
            .padding(rem(1))
            .cursor(CssCursor::Pointer),
        s()
            .hover()
            .background_color(Color::BackgroundLight2),
        attrs!{
            At::TabIndex => 0,
            At::Title => title,
        },
        div![
            C!["label"],
            s()
                .color(Color::SurfaceLight5_90)
                .flex("1")
                .max_height(em(4.8)),
            title,
        ],
        IF!(selected => {
            div![
                C!["icon"],
                s()
                    .display(CssDisplay::Block)
                    .background_color(Color::Accent3_90)
                    .border_radius(pc(100))
                    .flex(CssFlex::None)
                    .height(rem(0.5))
                    .margin_left(rem(1))
                    .width(rem(0.5))
            ]
        })
    ]
}
