use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

pub struct Model<T> {
    groups: Vec<Group<T>>
}

pub struct Group<T> {
    label: Option<String>,
    items: Vec<GroupItem<T>>,
    limit: usize,
    required: bool,
}

pub struct GroupItem<T> {
    label: String,
    value: T,
    selected: bool,
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


