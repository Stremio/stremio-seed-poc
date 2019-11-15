use seed::{prelude::*, *};

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
}

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model {
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    ItemClicked(GroupId, GroupItemId)
}

pub fn update<T: 'static, ParentMsg>(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, mut groups: Vec<Group<T>>, on_change: impl FnOnce(Vec<Group<T>>) -> ParentMsg) -> Option<ParentMsg> {
    match msg {
        Msg::ItemClicked(group_id, item_id) => {
            let group = groups.iter_mut().find(|group| group.id == group_id).unwrap();
            let selected_count = group.items.iter().filter(|item| item.selected).count();
            let first_selected_position = group.items.iter_mut().position(|item| item.selected);
            let item = group.items.iter_mut().find(|item| item.id == item_id).unwrap();

            // @TODO: Refactor + comments
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
                    if let Some(first_selected_position) = first_selected_position {
                        item.selected = true;
                        group.items.get_mut(first_selected_position).unwrap().selected = false;
                        true
                    } else {
                        false
                    }
                }
            };

            if changed {
                let groups_with_selected_items = groups.into_iter().map(|mut group| {
                    group.items = group.items.into_iter().filter(|item| item.selected).collect();
                    group
                }).collect::<Vec<_>>();
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
    div![
        class!["multi-select"],
        groups.iter().map(view_group)
    ]
}

pub fn view_group<T: Clone>(group: &Group<T>) -> Node<Msg> {
    div![
        class!["group"],
        match &group.label {
            Some(label) => label,
            None => "",
        },
        group.items.iter().map(|item| view_group_item(&group.id, item))
    ]
}

pub fn view_group_item<T: Clone>(group_id: &str, item: &GroupItem<T>) -> Node<Msg> {
    div![
        class!["group-item"],
        style!{
            St::Background => if item.selected { "green" } else { "initial" }
        },
        simple_ev(Ev::Click, Msg::ItemClicked(group_id.to_owned(), item.id.clone())),
        item.label,
    ]
}


