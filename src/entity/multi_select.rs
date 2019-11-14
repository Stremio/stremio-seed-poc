use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

pub struct Model<T> {
    groups: Vec<Group<T>>
}

pub struct Group<T> {
    pub label: Option<String>,
    pub items: Vec<GroupItem<T>>,
    pub limit: usize,
    pub required: bool,
}

pub struct GroupItem<T> {
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
}

pub fn update<T>(msg: Msg, model: &mut Model<T>, orders: &mut impl Orders<Msg>) {
    match msg {

    }
}

// ------ ------
//     View
// ------ ------

pub fn view<T>(model: &Model<T>) -> Node<Msg> {
    div![
        class!["multi-select"],
        "MULTISELEDT",
        model.groups.iter().map(view_group)
    ]
}

pub fn view_group<T>(group: &Group<T>) -> Node<Msg> {
    div![
        class!["group"],
        match &group.label {
            Some(label) => label,
            None => "",
        },
        group.items.iter().map(view_group_item)
    ]
}

pub fn view_group_item<T>(item: &GroupItem<T>) -> Node<Msg> {
    div![
        class!["group-item"],
        item.label,
    ]
}


