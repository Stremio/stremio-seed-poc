use crate::PageId;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(
    mut url: Url,
    model: &mut Option<Model>,
    orders: &mut impl Orders<Msg>,
) -> Option<PageId> {
    model.replace(Model {
        base_url: url.to_hash_base_url(),
        search_query: url.next_hash_path_part().unwrap_or_default().to_owned(),
    });
    Some(PageId::Search)
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    base_url: Url,
    search_query: String,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {

}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        "Search"
    ]
}
