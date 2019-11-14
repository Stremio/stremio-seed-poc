use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

pub type GroupId = String;
pub type GroupItemId = String;

pub struct Model<T> {
    groups: Vec<Group<T>>
}

pub struct Group<T> {
    pub id: GroupId,
    pub label: Option<String>,
    pub items: Vec<GroupItem<T>>,
    pub limit: usize,
    pub required: bool,
}

pub struct GroupItem<T> {
    pub id: GroupItemId,
    pub label: String,
    pub value: T,
    pub selected: bool,
}

// ------ ------
//     Init
// ------ ------

pub fn init<T>(groups: Vec<Group<T>>) -> Model<T> {
    Model {
        groups
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    ItemClicked(GroupId, GroupItemId)
}

pub fn update<T: 'static, ParentMsg>(msg: Msg, model: &mut Model<T>, orders: &mut impl Orders<Msg>, on_change: impl FnOnce() -> ParentMsg) -> Option<ParentMsg> {
    match msg {
        Msg::ItemClicked(group_id, item_id) => {
            let group = model.groups.iter_mut().find(|group| group.id == group_id).unwrap();
            let selected_count = group.items.iter().filter(|item| item.selected).count();
            let first_selected_position = group.items.iter_mut().position(|item| item.selected);
            let item = group.items.iter_mut().find(|item| item.id == item_id).unwrap();

            if item.selected {
                if !group.required || selected_count > 1 {
                    item.selected = false;
                }
            } else {
                if selected_count < group.limit {
                    item.selected = true;
                } else {
                    if let Some(first_selected_position) = first_selected_position {
                        item.selected = true;
                        group.items.get_mut(first_selected_position).unwrap().selected = false;
                    }
                }
            }
            Some(on_change())
        }
    }
}

pub fn set_groups<T: 'static>(groups: Vec<Group<T>>, model: &mut Model<T>, orders: &mut impl Orders<Msg>) {
    model.groups = groups;
}

// ------ ------
//     View
// ------ ------

pub fn view<T: Clone>(model: &Model<T>) -> Node<Msg> {
    div![
        class!["multi-select"],
        model.groups.iter().map(view_group)
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


